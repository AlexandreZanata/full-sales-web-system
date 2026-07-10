import { useCallback, useEffect, useMemo, useRef, useState, type ReactNode } from 'react';

import {
  PlatformAuthContext,
  type LoginStepResult,
  type PlatformAuthContextValue,
  type PlatformUserSummary,
} from '@/auth/usePlatformAuth';
import { PlatformLoginError, loginErrorMessage } from '@/lib/auth/authErrors';
import { resolveSessionFromAccessToken } from '@/lib/auth/authSession';
import { DEV_STUB_ACCESS, DEV_STUB_EMAIL, DEV_STUB_REFRESH } from '@/lib/auth/devStub';
import { decodeAccessTokenClaims, isPlatformAdminRole } from '@/lib/auth/jwt';
import {
  clearStoredUserEmail,
  getStoredUserEmail,
  setStoredUserEmail,
} from '@/lib/auth/sessionUser';
import {
  clearTokens,
  getAccessToken,
  getTokenExpiresAt,
  hasSession,
  msUntilTokenRefresh,
  setTokens,
} from '@/lib/auth/tokens';
import { ApiError, apiPost, setSessionExpiredHandler, tryRefreshTokens } from '@/lib/api/client';
import type { PlatformLoginResponse, TokenResponse } from '@/lib/api/types';

function toUserSummary(
  email: string,
  role: string,
  impersonating = false,
  actingTenantId?: string,
): PlatformUserSummary {
  return { email, role, impersonating, actingTenantId };
}

function applyTokens(email: string, tokens: TokenResponse): PlatformUserSummary {
  setTokens(tokens);
  setStoredUserEmail(email);
  const claims = decodeAccessTokenClaims(tokens.accessToken);
  if (!claims || !isPlatformAdminRole(claims.role)) {
    clearTokens();
    throw new PlatformLoginError(loginErrorMessage('NOT_PLATFORM_ADMIN'), 'NOT_PLATFORM_ADMIN');
  }
  return toUserSummary(email, claims.role, Boolean(claims.impersonating), claims.actingTenantId);
}

export function PlatformAuthProvider({ children }: { children: ReactNode }) {
  const [user, setUser] = useState<PlatformUserSummary | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const sessionPromise = useRef<Promise<PlatformUserSummary | null> | null>(null);
  const refreshTimer = useRef<ReturnType<typeof setTimeout> | null>(null);

  const clearRefreshTimer = useCallback(() => {
    if (refreshTimer.current) {
      clearTimeout(refreshTimer.current);
      refreshTimer.current = null;
    }
  }, []);

  const scheduleProactiveRefresh = useCallback(() => {
    clearRefreshTimer();
    const expiresAt = getTokenExpiresAt();
    if (!expiresAt) {
      return;
    }
    refreshTimer.current = setTimeout(() => {
      void (async () => {
        const refreshed = await tryRefreshTokens();
        if (refreshed) {
          scheduleProactiveRefresh();
        }
      })();
    }, msUntilTokenRefresh(expiresAt));
  }, [clearRefreshTimer]);

  const applySession = useCallback(
    (nextUser: PlatformUserSummary | null) => {
      setUser(nextUser);
      setIsLoading(false);
      if (nextUser) {
        scheduleProactiveRefresh();
      } else {
        clearRefreshTimer();
      }
    },
    [clearRefreshTimer, scheduleProactiveRefresh],
  );

  const loadSession = useCallback((): Promise<PlatformUserSummary | null> => {
    if (!hasSession()) {
      applySession(null);
      return Promise.resolve(null);
    }

    const accessToken = getAccessToken();
    if (!accessToken) {
      clearTokens();
      applySession(null);
      return Promise.resolve(null);
    }

    const restored = resolveSessionFromAccessToken(accessToken);
    if (restored.kind === 'missing' || restored.kind === 'invalid') {
      clearTokens();
      applySession(null);
      return Promise.resolve(null);
    }

    if (restored.needsRefresh) {
      return tryRefreshTokens().then((refreshed) => {
        if (!refreshed) {
          applySession(null);
          return null;
        }
        const token = getAccessToken();
        if (!token) {
          applySession(null);
          return null;
        }
        const afterRefresh = resolveSessionFromAccessToken(token);
        if (afterRefresh.kind !== 'ok') {
          clearTokens();
          applySession(null);
          return null;
        }
        const nextUser = toUserSummary(
          afterRefresh.user.email,
          afterRefresh.user.role,
          afterRefresh.user.impersonating,
          afterRefresh.user.actingTenantId,
        );
        applySession(nextUser);
        return nextUser;
      });
    }

    const nextUser = toUserSummary(
      restored.user.email,
      restored.user.role,
      restored.user.impersonating,
      restored.user.actingTenantId,
    );
    applySession(nextUser);
    return Promise.resolve(nextUser);
  }, [applySession]);

  const ensureSession = useCallback(async (): Promise<PlatformUserSummary | null> => {
    if (user) {
      return user;
    }
    if (!sessionPromise.current) {
      sessionPromise.current = loadSession().finally(() => {
        sessionPromise.current = null;
      });
    }
    return sessionPromise.current;
  }, [loadSession, user]);

  useEffect(() => {
    void ensureSession();
  }, [ensureSession]);

  useEffect(() => {
    setSessionExpiredHandler(() => {
      applySession(null);
    });
    return () => {
      setSessionExpiredHandler(null);
      clearRefreshTimer();
    };
  }, [applySession, clearRefreshTimer]);

  const enterDevShell = useCallback(() => {
    setTokens({ accessToken: DEV_STUB_ACCESS, refreshToken: DEV_STUB_REFRESH, expiresIn: 900 });
    setStoredUserEmail(DEV_STUB_EMAIL);
    applySession(toUserSummary(DEV_STUB_EMAIL, 'PlatformAdmin'));
  }, [applySession]);

  const login = useCallback(
    async (credentials: { email: string; password: string }): Promise<LoginStepResult> => {
      const email = credentials.email.trim().toLowerCase();
      let response: PlatformLoginResponse;
      try {
        response = await apiPost<PlatformLoginResponse>(
          '/platform/auth/login',
          { email, password: credentials.password },
          { skipAuth: true, skipRefreshRetry: true },
        );
      } catch (error) {
        if (error instanceof ApiError) {
          if (error.code === 'INVALID_CREDENTIALS' || error.status === 401) {
            throw new PlatformLoginError(
              loginErrorMessage('INVALID_CREDENTIALS'),
              'INVALID_CREDENTIALS',
            );
          }
          if (error.code === 'RATE_LIMITED' || error.status === 429) {
            throw new PlatformLoginError(loginErrorMessage('RATE_LIMITED'), 'RATE_LIMITED');
          }
        }
        throw new PlatformLoginError(loginErrorMessage('UNKNOWN'), 'UNKNOWN');
      }

      if ('mfaRequired' in response) {
        setStoredUserEmail(email);
        return { kind: 'mfa', mfaToken: response.mfaToken };
      }

      if (!('accessToken' in response)) {
        throw new PlatformLoginError(loginErrorMessage('UNKNOWN'), 'UNKNOWN');
      }

      const summary = applyTokens(email, response);
      applySession(summary);
      return { kind: 'tokens' };
    },
    [applySession],
  );

  const verifyMfa = useCallback(
    async (payload: { code: string; mfaToken: string }) => {
      const email = getStoredUserEmail() ?? '';
      let tokens: TokenResponse;
      try {
        tokens = await apiPost<TokenResponse>(
          '/platform/auth/mfa/verify',
          { code: payload.code.trim(), mfaToken: payload.mfaToken },
          { skipAuth: true, skipRefreshRetry: true },
        );
      } catch (error) {
        if (
          error instanceof ApiError &&
          (error.status === 401 || error.code === 'INVALID_CREDENTIALS')
        ) {
          throw new PlatformLoginError(loginErrorMessage('MFA_INVALID'), 'MFA_INVALID');
        }
        throw new PlatformLoginError(loginErrorMessage('UNKNOWN'), 'UNKNOWN');
      }
      const summary = applyTokens(email, tokens);
      applySession(summary);
    },
    [applySession],
  );

  const logout = useCallback(async () => {
    if (hasSession()) {
      try {
        await apiPost('/platform/auth/logout', {});
      } catch {
        // best-effort
      }
    }
    clearTokens();
    clearStoredUserEmail();
    applySession(null);
  }, [applySession]);

  const value = useMemo<PlatformAuthContextValue>(
    () => ({ user, isLoading, login, verifyMfa, logout, ensureSession, enterDevShell }),
    [user, isLoading, login, verifyMfa, logout, ensureSession, enterDevShell],
  );

  return <PlatformAuthContext.Provider value={value}>{children}</PlatformAuthContext.Provider>;
}

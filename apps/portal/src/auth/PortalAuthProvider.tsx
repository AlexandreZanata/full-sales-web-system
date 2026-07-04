import { useCallback, useEffect, useMemo, useRef, useState, type ReactNode } from 'react';

import { PortalLoginError, loginErrorMessage } from '@/lib/auth/authErrors';
import { resolveSessionFromAccessToken } from '@/lib/auth/authSession';
import { DEV_STUB_ACCESS, DEV_STUB_EMAIL, DEV_STUB_REFRESH } from '@/lib/auth/devStub';
import { decodeAccessTokenClaims, isCommerceContactRole } from '@/lib/auth/jwt';
import { clearStoredUserEmail, setStoredUserEmail } from '@/lib/auth/sessionUser';
import {
  clearTokens,
  getAccessToken,
  getTokenExpiresAt,
  hasSession,
  msUntilTokenRefresh,
  setTokens,
} from '@/lib/auth/tokens';
import { ApiError, apiPost, setSessionExpiredHandler, tryRefreshTokens } from '@/lib/api/client';
import type { TokenResponse } from '@/lib/api/types';

import {
  PortalAuthContext,
  type PortalAuthContextValue,
  type PortalUserSummary,
} from './usePortalAuth';

function toUserSummary(email: string, role: string, commerceId?: string): PortalUserSummary {
  return { email, role, commerceId };
}

export function PortalAuthProvider({ children }: { children: ReactNode }) {
  const [user, setUser] = useState<PortalUserSummary | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const sessionPromise = useRef<Promise<PortalUserSummary | null> | null>(null);
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
    (nextUser: PortalUserSummary | null) => {
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

  const loadSession = useCallback((): Promise<PortalUserSummary | null> => {
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
          afterRefresh.user.commerceId,
        );
        applySession(nextUser);
        return nextUser;
      });
    }

    const nextUser = toUserSummary(
      restored.user.email,
      restored.user.role,
      restored.user.commerceId,
    );
    applySession(nextUser);
    return Promise.resolve(nextUser);
  }, [applySession]);

  const ensureSession = useCallback(async (): Promise<PortalUserSummary | null> => {
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
    applySession(
      toUserSummary(DEV_STUB_EMAIL, 'CommerceContact', '01900001-0010-7000-8000-000000000001'),
    );
  }, [applySession]);

  const login = useCallback(
    async (credentials: { email: string; password: string }) => {
      const email = credentials.email.trim().toLowerCase();

      let tokens: TokenResponse;
      try {
        tokens = await apiPost<TokenResponse>(
          '/auth/login',
          { email, password: credentials.password },
          { skipAuth: true, skipRefreshRetry: true },
        );
      } catch (error) {
        if (error instanceof ApiError) {
          if (error.code === 'INVALID_CREDENTIALS' || error.status === 401) {
            throw new PortalLoginError(
              loginErrorMessage('INVALID_CREDENTIALS'),
              'INVALID_CREDENTIALS',
            );
          }
          if (error.code === 'RATE_LIMITED' || error.status === 429) {
            throw new PortalLoginError(loginErrorMessage('RATE_LIMITED'), 'RATE_LIMITED');
          }
        }
        throw new PortalLoginError(loginErrorMessage('UNKNOWN'), 'UNKNOWN');
      }

      const claims = decodeAccessTokenClaims(tokens.accessToken);
      if (!claims || !isCommerceContactRole(claims.role)) {
        clearTokens();
        throw new PortalLoginError(loginErrorMessage('NOT_PORTAL_USER'), 'NOT_PORTAL_USER');
      }

      setTokens(tokens);
      setStoredUserEmail(email);
      applySession(toUserSummary(email, claims.role, claims.commerceId));
    },
    [applySession],
  );

  const logout = useCallback(async () => {
    if (hasSession()) {
      try {
        await apiPost('/auth/logout', {});
      } catch {
        // best-effort
      }
    }
    clearTokens();
    clearStoredUserEmail();
    applySession(null);
  }, [applySession]);

  const value = useMemo<PortalAuthContextValue>(
    () => ({ user, isLoading, login, logout, ensureSession, enterDevShell }),
    [user, isLoading, login, logout, ensureSession, enterDevShell],
  );

  return <PortalAuthContext.Provider value={value}>{children}</PortalAuthContext.Provider>;
}

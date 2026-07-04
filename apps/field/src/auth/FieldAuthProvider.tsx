import { useCallback, useEffect, useMemo, useRef, useState, type ReactNode } from 'react';

import { FieldLoginError, loginErrorMessage } from '@/lib/auth/authErrors';
import { resolveSessionFromAccessToken } from '@/lib/auth/authSession';
import { DEV_STUB_ACCESS, DEV_STUB_EMAIL, DEV_STUB_REFRESH } from '@/lib/auth/devStub';
import { decodeAccessTokenClaims, isFieldRole } from '@/lib/auth/jwt';
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

import {
  FieldAuthContext,
  type FieldAuthContextValue,
  type FieldUserSummary,
} from './useFieldAuth';

function toUserSummary(email: string, role: string): FieldUserSummary {
  return { email, role };
}

export function FieldAuthProvider({ children }: { children: ReactNode }) {
  const [user, setUser] = useState<FieldUserSummary | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const sessionPromise = useRef<Promise<FieldUserSummary | null> | null>(null);
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
    if (!expiresAt) return;
    refreshTimer.current = setTimeout(() => {
      void (async () => {
        if (await tryRefreshTokens()) scheduleProactiveRefresh();
      })();
    }, msUntilTokenRefresh(expiresAt));
  }, [clearRefreshTimer]);

  const applySession = useCallback(
    (nextUser: FieldUserSummary | null) => {
      setUser(nextUser);
      setIsLoading(false);
      if (nextUser) scheduleProactiveRefresh();
      else clearRefreshTimer();
    },
    [clearRefreshTimer, scheduleProactiveRefresh],
  );

  const loadSession = useCallback((): Promise<FieldUserSummary | null> => {
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
        const nextUser = toUserSummary(afterRefresh.user.email, afterRefresh.user.role);
        applySession(nextUser);
        return nextUser;
      });
    }
    const nextUser = toUserSummary(restored.user.email, restored.user.role);
    applySession(nextUser);
    return Promise.resolve(nextUser);
  }, [applySession]);

  const ensureSession = useCallback(async (): Promise<FieldUserSummary | null> => {
    if (user) return user;
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
    applySession(toUserSummary(DEV_STUB_EMAIL, 'Seller'));
  }, [applySession]);

  const login = useCallback(
    async (credentials: { email: string; password: string }) => {
      const email = credentials.email.trim().toLowerCase();
      let tokens: { accessToken: string; refreshToken: string; expiresIn: number };
      try {
        tokens = await apiPost(
          '/auth/login',
          { email, password: credentials.password },
          {
            skipAuth: true,
            skipRefreshRetry: true,
          },
        );
      } catch (error) {
        if (error instanceof ApiError) {
          if (error.code === 'INVALID_CREDENTIALS' || error.status === 401) {
            throw new FieldLoginError(
              loginErrorMessage('INVALID_CREDENTIALS'),
              'INVALID_CREDENTIALS',
            );
          }
          if (error.code === 'RATE_LIMITED' || error.status === 429) {
            throw new FieldLoginError(loginErrorMessage('RATE_LIMITED'), 'RATE_LIMITED');
          }
        }
        throw new FieldLoginError(loginErrorMessage('UNKNOWN'), 'UNKNOWN');
      }
      const claims = decodeAccessTokenClaims(tokens.accessToken);
      if (!claims || !isFieldRole(claims.role)) {
        clearTokens();
        throw new FieldLoginError(loginErrorMessage('NOT_FIELD_USER'), 'NOT_FIELD_USER');
      }
      setTokens(tokens);
      setStoredUserEmail(email);
      applySession(toUserSummary(email, claims.role));
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

  const value = useMemo<FieldAuthContextValue>(
    () => ({ user, isLoading, login, logout, ensureSession, enterDevShell }),
    [user, isLoading, login, logout, ensureSession, enterDevShell],
  );

  return <FieldAuthContext.Provider value={value}>{children}</FieldAuthContext.Provider>;
}

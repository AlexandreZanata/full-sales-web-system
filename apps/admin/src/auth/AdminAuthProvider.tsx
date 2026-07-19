import { useCallback, useEffect, useMemo, useRef, useState, type ReactNode } from 'react';

import { AdminLoginError, loginErrorMessage } from '@/lib/auth/authErrors';
import { DEV_STUB_ACCESS, DEV_STUB_EMAIL, DEV_STUB_REFRESH } from '@/lib/auth/devStub';
import { decodeAccessTokenClaims, isAdminRole } from '@/lib/auth/jwt';
import { restoreAdminSession } from '@/lib/auth/restoreAdminSession';
import { clearStoredUserEmail, setStoredUserEmail } from '@/lib/auth/sessionUser';
import {
  clearTokens,
  getTokenExpiresAt,
  hasSession,
  msUntilTokenRefresh,
  setTokens,
} from '@/lib/auth/tokens';
import { ApiError, apiPost, setSessionExpiredHandler, tryRefreshTokens } from '@/lib/api/client';
import type { TokenResponse } from '@/lib/api/types';

import {
  AdminAuthContext,
  type AdminAuthContextValue,
  type AdminUserSummary,
} from './useAdminAuth';

function toUserSummary(email: string, role: string): AdminUserSummary {
  return { email, role };
}

export function AdminAuthProvider({ children }: { children: ReactNode }) {
  const [user, setUser] = useState<AdminUserSummary | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const sessionPromise = useRef<Promise<AdminUserSummary | null> | null>(null);
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
    (nextUser: AdminUserSummary | null) => {
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

  const loadSession = useCallback((): Promise<AdminUserSummary | null> => {
    return restoreAdminSession().then((restored) => {
      const next = restored ? toUserSummary(restored.email, restored.role) : null;
      applySession(next);
      return next;
    });
  }, [applySession]);

  const ensureSession = useCallback(async (): Promise<AdminUserSummary | null> => {
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
    applySession(toUserSummary(DEV_STUB_EMAIL, 'Admin'));
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
            throw new AdminLoginError(
              loginErrorMessage('INVALID_CREDENTIALS'),
              'INVALID_CREDENTIALS',
            );
          }
          if (error.code === 'RATE_LIMITED' || error.status === 429) {
            throw new AdminLoginError(loginErrorMessage('RATE_LIMITED'), 'RATE_LIMITED');
          }
        }
        throw new AdminLoginError(loginErrorMessage('UNKNOWN'), 'UNKNOWN');
      }

      const claims = decodeAccessTokenClaims(tokens.accessToken);
      if (!claims || !isAdminRole(claims.role)) {
        clearTokens();
        throw new AdminLoginError(loginErrorMessage('NOT_ADMIN'), 'NOT_ADMIN');
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
        // best-effort server logout
      }
    }
    clearTokens();
    clearStoredUserEmail();
    applySession(null);
  }, [applySession]);

  const value = useMemo<AdminAuthContextValue>(
    () => ({ user, isLoading, login, logout, ensureSession, enterDevShell }),
    [user, isLoading, login, logout, ensureSession, enterDevShell],
  );

  return <AdminAuthContext.Provider value={value}>{children}</AdminAuthContext.Provider>;
}

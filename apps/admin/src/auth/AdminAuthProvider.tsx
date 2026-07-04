import { useCallback, useEffect, useMemo, useRef, useState, type ReactNode } from 'react';

import { apiPost } from '@/lib/api/client';
import { clearTokens, hasSession, setTokens } from '@/lib/auth/tokens';

import {
  AdminAuthContext,
  type AdminAuthContextValue,
  type AdminUserSummary,
} from './useAdminAuth';

const DEV_STUB_ACCESS = 'dev-stub-access';
const DEV_STUB_REFRESH = 'dev-stub-refresh';

function stubUser(): AdminUserSummary {
  return { email: 'admin@localhost' };
}

export function AdminAuthProvider({ children }: { children: ReactNode }) {
  const [user, setUser] = useState<AdminUserSummary | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const sessionPromise = useRef<Promise<AdminUserSummary | null> | null>(null);

  const loadSession = useCallback((): Promise<AdminUserSummary | null> => {
    if (!hasSession()) {
      setUser(null);
      setIsLoading(false);
      return Promise.resolve(null);
    }

    const nextUser = stubUser();
    setUser(nextUser);
    setIsLoading(false);
    return Promise.resolve(nextUser);
  }, []);

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

  const enterDevShell = useCallback(() => {
    setTokens({ accessToken: DEV_STUB_ACCESS, refreshToken: DEV_STUB_REFRESH });
    setUser(stubUser());
    setIsLoading(false);
  }, []);

  const login = useCallback((): Promise<void> => {
    return Promise.reject(new Error('Login is implemented in Phase 29'));
  }, []);

  const logout = useCallback(async () => {
    const token = hasSession();
    if (token) {
      try {
        await apiPost('/auth/logout', {});
      } catch {
        // ignore logout errors during stub phase
      }
    }
    clearTokens();
    setUser(null);
  }, []);

  const value = useMemo<AdminAuthContextValue>(
    () => ({ user, isLoading, login, logout, ensureSession, enterDevShell }),
    [user, isLoading, login, logout, ensureSession, enterDevShell],
  );

  return <AdminAuthContext.Provider value={value}>{children}</AdminAuthContext.Provider>;
}

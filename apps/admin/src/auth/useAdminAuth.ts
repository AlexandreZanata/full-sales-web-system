import { createContext, useContext } from 'react';

export type AdminUserSummary = {
  email: string;
  role: string;
};

export type AdminAuthContextValue = {
  user: AdminUserSummary | null;
  isLoading: boolean;
  login: (credentials: { email: string; password: string }) => Promise<void>;
  logout: () => Promise<void>;
  ensureSession: () => Promise<AdminUserSummary | null>;
  /** Dev stub — sets placeholder tokens until Phase 29 login is wired. */
  enterDevShell: () => void;
};

export const AdminAuthContext = createContext<AdminAuthContextValue | null>(null);

export function useAdminAuth(): AdminAuthContextValue {
  const context = useContext(AdminAuthContext);
  if (!context) {
    throw new Error('useAdminAuth must be used within AdminAuthProvider');
  }
  return context;
}

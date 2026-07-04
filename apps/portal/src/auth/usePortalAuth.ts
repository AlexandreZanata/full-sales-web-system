import { createContext, useContext } from 'react';

export type PortalUserSummary = {
  email: string;
  role: string;
  commerceId?: string;
};

export type PortalAuthContextValue = {
  user: PortalUserSummary | null;
  isLoading: boolean;
  login: (credentials: { email: string; password: string }) => Promise<void>;
  logout: () => Promise<void>;
  ensureSession: () => Promise<PortalUserSummary | null>;
  enterDevShell: () => void;
};

export const PortalAuthContext = createContext<PortalAuthContextValue | null>(null);

export function usePortalAuth(): PortalAuthContextValue {
  const context = useContext(PortalAuthContext);
  if (!context) {
    throw new Error('usePortalAuth must be used within PortalAuthProvider');
  }
  return context;
}

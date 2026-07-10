import { createContext, useContext } from 'react';

export type PlatformUserSummary = {
  email: string;
  role: string;
  impersonating: boolean;
  actingTenantId?: string;
};

export type PlatformAuthContextValue = {
  user: PlatformUserSummary | null;
  isLoading: boolean;
  login: (credentials: { email: string; password: string }) => Promise<LoginStepResult>;
  verifyMfa: (payload: { code: string; mfaToken: string }) => Promise<void>;
  logout: () => Promise<void>;
  ensureSession: () => Promise<PlatformUserSummary | null>;
  enterDevShell: () => void;
};

export type LoginStepResult = { kind: 'tokens' } | { kind: 'mfa'; mfaToken: string };

export const PlatformAuthContext = createContext<PlatformAuthContextValue | null>(null);

export function usePlatformAuth(): PlatformAuthContextValue {
  const context = useContext(PlatformAuthContext);
  if (!context) {
    throw new Error('usePlatformAuth must be used within PlatformAuthProvider');
  }
  return context;
}

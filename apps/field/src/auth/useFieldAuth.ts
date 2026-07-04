import { createContext, useContext } from 'react';

export type FieldUserSummary = {
  email: string;
  role: string;
};

export type FieldAuthContextValue = {
  user: FieldUserSummary | null;
  isLoading: boolean;
  login: (credentials: { email: string; password: string }) => Promise<void>;
  logout: () => Promise<void>;
  ensureSession: () => Promise<FieldUserSummary | null>;
  enterDevShell: () => void;
};

export const FieldAuthContext = createContext<FieldAuthContextValue | null>(null);

export function useFieldAuth(): FieldAuthContextValue {
  const context = useContext(FieldAuthContext);
  if (!context) {
    throw new Error('useFieldAuth must be used within FieldAuthProvider');
  }
  return context;
}

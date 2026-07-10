import { createContext } from 'react';

export type ToastContextValue = {
  success: (message: string) => void;
  error: (message: string) => void;
};

export const ToastContext = createContext<ToastContextValue | null>(null);

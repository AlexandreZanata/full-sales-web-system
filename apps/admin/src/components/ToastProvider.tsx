import { useCallback, useMemo, useState, type ReactNode } from 'react';

import { randomId } from '@/lib/randomId';
import { ToastContext } from '@/lib/toastContext';
import { cn } from '@/lib/utils';

type ToastVariant = 'success' | 'error';

type ToastItem = {
  id: string;
  message: string;
  variant: ToastVariant;
};

export function ToastProvider({ children }: { children: ReactNode }) {
  const [toasts, setToasts] = useState<ToastItem[]>([]);

  const dismiss = useCallback((id: string) => {
    setToasts((current) => current.filter((toast) => toast.id !== id));
  }, []);

  const push = useCallback(
    (message: string, variant: ToastVariant) => {
      const id = randomId();
      setToasts((current) => [...current, { id, message, variant }]);
      window.setTimeout(() => {
        dismiss(id);
      }, 4000);
    },
    [dismiss],
  );

  const value = useMemo(
    () => ({
      success: (message: string) => {
        push(message, 'success');
      },
      error: (message: string) => {
        push(message, 'error');
      },
    }),
    [push],
  );

  return (
    <ToastContext.Provider value={value}>
      {children}
      <div
        aria-live="polite"
        className="pointer-events-none fixed bottom-4 right-4 z-50 flex max-w-sm flex-col gap-2"
      >
        {toasts.map((toast) => (
          <div
            key={toast.id}
            role="status"
            className={cn(
              'rounded-md border px-4 py-3 text-sm shadow-sm',
              toast.variant === 'success'
                ? 'border-admin-accent/30 bg-surface text-foreground'
                : 'border-destructive/30 bg-destructive/5 text-destructive',
            )}
          >
            {toast.message}
          </div>
        ))}
      </div>
    </ToastContext.Provider>
  );
}

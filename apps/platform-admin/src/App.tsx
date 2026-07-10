import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { RouterProvider } from '@tanstack/react-router';
import { useMemo } from 'react';

import { PlatformAuthProvider } from '@/auth/PlatformAuthProvider';
import { usePlatformAuth } from '@/auth/usePlatformAuth';
import { ToastProvider } from '@/components/ToastProvider';
import { I18nProvider } from '@/lib/i18n/context';
import { router } from '@/router';

function PlatformRouter() {
  const auth = usePlatformAuth();
  const queryClient = useMemo(() => new QueryClient(), []);

  return (
    <QueryClientProvider client={queryClient}>
      <RouterProvider router={router} context={{ auth, queryClient }} />
    </QueryClientProvider>
  );
}

export function App() {
  return (
    <I18nProvider>
      <ToastProvider>
        <PlatformAuthProvider>
          <PlatformRouter />
        </PlatformAuthProvider>
      </ToastProvider>
    </I18nProvider>
  );
}

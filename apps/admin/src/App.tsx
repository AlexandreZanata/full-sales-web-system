import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { RouterProvider } from '@tanstack/react-router';
import { useMemo } from 'react';

import { AdminAuthProvider } from '@/auth/AdminAuthProvider';
import { useAdminAuth } from '@/auth/useAdminAuth';
import { ToastProvider } from '@/components/ToastProvider';
import { useCatalogRealtime } from '@/lib/catalog/useCatalogRealtime';
import { I18nProvider } from '@/lib/i18n/context';
import { router } from '@/router';

function AdminRouter() {
  const auth = useAdminAuth();
  const queryClient = useMemo(() => new QueryClient(), []);
  useCatalogRealtime(queryClient);

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
        <AdminAuthProvider>
          <AdminRouter />
        </AdminAuthProvider>
      </ToastProvider>
    </I18nProvider>
  );
}

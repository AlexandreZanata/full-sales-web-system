import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { RouterProvider } from '@tanstack/react-router';
import { useMemo } from 'react';

import { FieldAuthProvider } from '@/auth/FieldAuthProvider';
import { useFieldAuth } from '@/auth/useFieldAuth';
import { useCatalogRealtime } from '@/lib/catalog/useCatalogRealtime';
import { I18nProvider } from '@/lib/i18n/context';
import { router } from '@/router';

function FieldRouter() {
  const auth = useFieldAuth();
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
      <FieldAuthProvider>
        <FieldRouter />
      </FieldAuthProvider>
    </I18nProvider>
  );
}

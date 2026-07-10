import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { RouterProvider } from '@tanstack/react-router';
import { useEffect, useMemo } from 'react';

import { PortalAuthProvider } from '@/auth/PortalAuthProvider';
import { usePortalAuth } from '@/auth/usePortalAuth';
import { CartProvider } from '@/cart/CartProvider';
import { useCatalogRealtime } from '@/lib/catalog/useCatalogRealtime';
import { I18nProvider } from '@/lib/i18n/context';
import { router } from '@/router';
import { bootstrapPortalTheme } from '@/lib/settings/applyTheme';

function PortalRouter() {
  const auth = usePortalAuth();
  const queryClient = useMemo(() => new QueryClient(), []);
  useCatalogRealtime(queryClient);

  return (
    <QueryClientProvider client={queryClient}>
      <RouterProvider router={router} context={{ auth, queryClient }} />
    </QueryClientProvider>
  );
}

export function App() {
  useEffect(() => {
    void bootstrapPortalTheme();
  }, []);

  return (
    <I18nProvider>
      <PortalAuthProvider>
        <CartProvider>
          <PortalRouter />
        </CartProvider>
      </PortalAuthProvider>
    </I18nProvider>
  );
}

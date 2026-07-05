import type { QueryClient } from '@tanstack/react-query';
import { useEffect } from 'react';

const CATALOG_SSE_EVENT = 'catalog.changed';

export function invalidatePortalCatalogQueries(queryClient: QueryClient): void {
  void queryClient.invalidateQueries({
    queryKey: ['portal', 'products'],
    refetchType: 'active',
  });
  void queryClient.invalidateQueries({
    queryKey: ['portal', 'categories'],
    refetchType: 'active',
  });
  void queryClient.invalidateQueries({
    queryKey: ['portal', 'category'],
    refetchType: 'active',
  });
  void queryClient.invalidateQueries({
    queryKey: ['portal', 'product'],
    refetchType: 'active',
  });
}

export function useCatalogRealtime(queryClient: QueryClient): void {
  useEffect(() => {
    if (typeof window === 'undefined' || typeof EventSource === 'undefined') {
      return;
    }

    const source = new EventSource('/v1/public/catalog/events');
    const onCatalogChanged = () => {
      invalidatePortalCatalogQueries(queryClient);
    };

    source.addEventListener(CATALOG_SSE_EVENT, onCatalogChanged);
    return () => {
      source.removeEventListener(CATALOG_SSE_EVENT, onCatalogChanged);
      source.close();
    };
  }, [queryClient]);
}

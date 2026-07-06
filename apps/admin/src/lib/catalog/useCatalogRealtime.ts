import type { QueryClient } from '@tanstack/react-query';
import { useEffect } from 'react';

const CATALOG_SSE_EVENT = 'catalog.changed';

const revisionListeners = new Set<() => void>();

export function subscribeCatalogRevision(listener: () => void): () => void {
  revisionListeners.add(listener);
  return () => {
    revisionListeners.delete(listener);
  };
}

export function bumpCatalogRevision(): void {
  revisionListeners.forEach((listener) => {
    listener();
  });
}

export function invalidateAdminCatalogQueries(queryClient: QueryClient): void {
  void queryClient.invalidateQueries({ queryKey: ['products'], refetchType: 'active' });
  void queryClient.invalidateQueries({ queryKey: ['categories'], refetchType: 'active' });
  void queryClient.invalidateQueries({
    queryKey: ['inventory', 'balances'],
    refetchType: 'active',
  });
}

export function onAdminCatalogChanged(queryClient: QueryClient): void {
  bumpCatalogRevision();
  invalidateAdminCatalogQueries(queryClient);
}

export function useCatalogRealtime(queryClient: QueryClient): void {
  useEffect(() => {
    if (typeof window === 'undefined' || typeof EventSource === 'undefined') {
      return;
    }

    const source = new EventSource('/v1/public/catalog/events');
    const onCatalogChanged = () => {
      onAdminCatalogChanged(queryClient);
    };

    source.addEventListener(CATALOG_SSE_EVENT, onCatalogChanged);
    return () => {
      source.removeEventListener(CATALOG_SSE_EVENT, onCatalogChanged);
      source.close();
    };
  }, [queryClient]);
}

import { QueryClient } from '@tanstack/react-query';
import { describe, expect, it, vi } from 'vitest';

import {
  bumpCatalogRevision,
  invalidateAdminCatalogQueries,
  onAdminCatalogChanged,
  subscribeCatalogRevision,
} from '@/lib/catalog/useCatalogRealtime';

describe('invalidateAdminCatalogQueries', () => {
  it('invalidates active product queries', () => {
    const queryClient = new QueryClient();
    const invalidateSpy = vi.spyOn(queryClient, 'invalidateQueries');

    invalidateAdminCatalogQueries(queryClient);

    expect(invalidateSpy).toHaveBeenCalledWith({
      queryKey: ['products'],
      refetchType: 'active',
    });
    expect(invalidateSpy).toHaveBeenCalledWith({
      queryKey: ['categories'],
      refetchType: 'active',
    });
    expect(invalidateSpy).toHaveBeenCalledWith({
      queryKey: ['inventory', 'balances'],
      refetchType: 'active',
    });
  });
});

describe('onAdminCatalogChanged', () => {
  it('bumps catalog revision listeners and invalidates queries', () => {
    const queryClient = new QueryClient();
    const invalidateSpy = vi.spyOn(queryClient, 'invalidateQueries');
    const listener = vi.fn();
    const unsubscribe = subscribeCatalogRevision(listener);

    onAdminCatalogChanged(queryClient);

    expect(listener).toHaveBeenCalledTimes(1);
    expect(invalidateSpy).toHaveBeenCalledWith({
      queryKey: ['categories'],
      refetchType: 'active',
    });

    unsubscribe();
    bumpCatalogRevision();
    expect(listener).toHaveBeenCalledTimes(1);
  });
});

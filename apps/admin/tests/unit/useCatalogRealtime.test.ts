import { QueryClient } from '@tanstack/react-query';
import { describe, expect, it, vi } from 'vitest';

import { invalidateAdminCatalogQueries } from '@/lib/catalog/useCatalogRealtime';

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

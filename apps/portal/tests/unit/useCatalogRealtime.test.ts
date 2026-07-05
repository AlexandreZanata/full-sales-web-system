import { QueryClient } from '@tanstack/react-query';
import { describe, expect, it, vi } from 'vitest';

import { invalidatePortalCatalogQueries } from '@/lib/catalog/useCatalogRealtime';

describe('invalidatePortalCatalogQueries', () => {
  it('invalidates active portal product queries', () => {
    const queryClient = new QueryClient();
    const invalidateSpy = vi.spyOn(queryClient, 'invalidateQueries');

    invalidatePortalCatalogQueries(queryClient);

    expect(invalidateSpy).toHaveBeenCalledWith({
      queryKey: ['portal', 'products'],
      refetchType: 'active',
    });
    expect(invalidateSpy).toHaveBeenCalledWith({
      queryKey: ['portal', 'categories'],
      refetchType: 'active',
    });
    expect(invalidateSpy).toHaveBeenCalledWith({
      queryKey: ['portal', 'category'],
      refetchType: 'active',
    });
    expect(invalidateSpy).toHaveBeenCalledWith({
      queryKey: ['portal', 'product'],
      refetchType: 'active',
    });
  });
});

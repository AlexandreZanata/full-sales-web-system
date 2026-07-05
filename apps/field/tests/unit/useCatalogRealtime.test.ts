import { QueryClient } from '@tanstack/react-query';
import { describe, expect, it, vi } from 'vitest';

import { invalidateFieldCatalogQueries } from '@/lib/catalog/useCatalogRealtime';

describe('invalidateFieldCatalogQueries', () => {
  it('invalidates active product queries', () => {
    const queryClient = new QueryClient();
    const invalidateSpy = vi.spyOn(queryClient, 'invalidateQueries');

    invalidateFieldCatalogQueries(queryClient);

    expect(invalidateSpy).toHaveBeenCalledWith({
      queryKey: ['products'],
      refetchType: 'active',
    });
  });
});

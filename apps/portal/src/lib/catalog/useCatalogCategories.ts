import { useQuery } from '@tanstack/react-query';

import { fetchPortalCategories } from '@/lib/api/portal';

const CATALOG_STALE_TIME_MS = 5 * 60 * 1000;

export function catalogCategoriesQueryKey() {
  return ['portal', 'categories'] as const;
}

export function useCatalogCategories(enabled = true) {
  return useQuery({
    queryKey: catalogCategoriesQueryKey(),
    queryFn: fetchPortalCategories,
    enabled,
    staleTime: CATALOG_STALE_TIME_MS,
  });
}

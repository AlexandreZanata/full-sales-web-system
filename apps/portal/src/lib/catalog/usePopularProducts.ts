import { useQuery } from '@tanstack/react-query';

import { fetchPortalPopularProducts } from '@/lib/api/portalPopular';

export const popularProductsQueryKey = (limit = 12) =>
  ['portal', 'products', 'popular', limit] as const;

export function usePopularProducts(limit = 12) {
  return useQuery({
    queryKey: popularProductsQueryKey(limit),
    queryFn: () => fetchPortalPopularProducts(limit),
    staleTime: 5 * 60 * 1000,
  });
}

import { useQuery } from '@tanstack/react-query';

import { fetchPortalFeaturedProducts } from '@/lib/api/portalFeatured';

export const featuredProductsQueryKey = (limit = 12) =>
  ['portal', 'products', 'featured', limit] as const;

export function useFeaturedProducts(limit = 12) {
  return useQuery({
    queryKey: featuredProductsQueryKey(limit),
    queryFn: () => fetchPortalFeaturedProducts(limit),
    staleTime: 5 * 60 * 1000,
  });
}

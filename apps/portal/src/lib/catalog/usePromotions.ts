import { useQuery } from '@tanstack/react-query';

import { fetchPortalPromotions } from '@/lib/api/portalPromotions';

export const promotionsQueryKey = (limit = 4) => ['portal', 'promotions', limit] as const;

export function usePromotions(limit = 4) {
  return useQuery({
    queryKey: promotionsQueryKey(limit),
    queryFn: () => fetchPortalPromotions(limit),
    staleTime: 5 * 60 * 1000,
  });
}

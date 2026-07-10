import { useQuery } from '@tanstack/react-query';

import { fetchPortalBanners } from '@/lib/api/portal';

export const heroBannersQueryKey = (placement = 'hero') =>
  ['portal', 'banners', placement] as const;

export function useHeroBanners(placement = 'hero') {
  return useQuery({
    queryKey: heroBannersQueryKey(placement),
    queryFn: () => fetchPortalBanners(placement),
    staleTime: 5 * 60 * 1000,
  });
}

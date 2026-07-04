import { useQuery } from '@tanstack/react-query';

import { fetchSettings } from '@/lib/api/settings';

export function useSiteSettings(enabled: boolean) {
  return useQuery({
    queryKey: ['settings'],
    queryFn: fetchSettings,
    enabled,
    staleTime: 5 * 60 * 1000,
  });
}

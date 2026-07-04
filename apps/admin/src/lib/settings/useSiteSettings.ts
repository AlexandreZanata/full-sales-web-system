import { useQuery } from '@tanstack/react-query';

import { fetchSettings } from '@/lib/api/settings';

const SETTINGS_QUERY_KEY = ['settings'] as const;

export function siteSettingsQueryKey() {
  return SETTINGS_QUERY_KEY;
}

export function useSiteSettings() {
  return useQuery({
    queryKey: SETTINGS_QUERY_KEY,
    queryFn: fetchSettings,
    staleTime: 5 * 60 * 1000,
  });
}

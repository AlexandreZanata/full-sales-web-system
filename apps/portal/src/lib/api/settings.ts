import { getAccessToken } from '@/lib/auth/tokens';
import { apiFetch } from '@/lib/api/client';

export type SiteSettings = {
  displayName: string;
  logoFileId?: string;
  logoUrl?: string;
  salesContactPhone?: string;
};

export async function fetchSettings(): Promise<SiteSettings> {
  const hasSession = Boolean(getAccessToken());
  const path = hasSession ? '/settings' : '/public/settings';
  return apiFetch<SiteSettings>(path, hasSession ? undefined : { skipAuth: true });
}

import { apiFetch } from '@/lib/api/client';

export type SiteSettings = {
  displayName: string;
  logoFileId?: string;
  logoUrl?: string;
};

export async function fetchSettings(): Promise<SiteSettings> {
  return apiFetch<SiteSettings>('/settings');
}

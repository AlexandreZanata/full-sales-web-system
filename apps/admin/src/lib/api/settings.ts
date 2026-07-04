import { apiFetch, apiPatch, apiPut } from '@/lib/api/client';

export type SiteSettings = {
  displayName: string;
  logoFileId?: string;
  logoUrl?: string;
};

export async function fetchSettings(): Promise<SiteSettings> {
  return apiFetch<SiteSettings>('/settings');
}

export async function updateSettings(body: { displayName: string }): Promise<SiteSettings> {
  return apiPatch<SiteSettings>('/settings', {
    body: JSON.stringify(body),
  });
}

export async function updateSiteLogo(fileId: string): Promise<SiteSettings> {
  return apiPut<SiteSettings>('/settings/logo', { fileId });
}

import { apiFetch } from '@/lib/api/client';

export type PublicSeller = {
  publicCode: string;
  displayName: string;
  contactPhone?: string;
};

export async function fetchPublicSeller(publicCode: string): Promise<PublicSeller> {
  return apiFetch<PublicSeller>(`/public/sellers/${encodeURIComponent(publicCode)}`, {
    skipAuth: true,
  });
}

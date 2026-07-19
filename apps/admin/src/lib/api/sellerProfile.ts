import { apiFetch, apiPut } from '@/lib/api/client';
import type { SellerProfile, SellerProfileRequest } from '@/lib/api/types';

export async function fetchSellerProfile(id: string): Promise<SellerProfile> {
  return apiFetch<SellerProfile>(`/users/${id}/seller-profile`);
}

export async function upsertSellerProfile(
  id: string,
  body: SellerProfileRequest,
): Promise<SellerProfile> {
  return apiPut<SellerProfile>(`/users/${id}/seller-profile`, body);
}

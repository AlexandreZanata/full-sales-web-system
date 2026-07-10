import { apiDelete, apiFetch, apiPatch, apiPost } from '@/lib/api/client';
import type { CursorListResponse } from '@/lib/api/types';

export type PortalBanner = {
  id: string;
  placement: string;
  imageFileId: string;
  linkUrl?: string;
  altText?: string;
  sortOrder: number;
  active: boolean;
};

export type PortalPromotion = {
  id: string;
  headline: string;
  discountText: string;
  background: 'yellow' | 'green';
  categorySlug?: string;
  linkUrl?: string;
  imageFileId?: string;
  sortOrder: number;
  active: boolean;
};

export type CreatePortalBannerRequest = {
  placement?: string;
  imageFileId: string;
  linkUrl?: string;
  altText?: string;
  sortOrder?: number;
  active?: boolean;
};

export type UpdatePortalBannerRequest = Partial<CreatePortalBannerRequest>;

export type CreatePortalPromotionRequest = {
  headline: string;
  discountText: string;
  background: 'yellow' | 'green';
  categorySlug?: string;
  linkUrl?: string;
  imageFileId?: string;
  sortOrder?: number;
  active?: boolean;
};

export type UpdatePortalPromotionRequest = Partial<CreatePortalPromotionRequest>;

export async function fetchPortalBanners(): Promise<PortalBanner[]> {
  const response = await apiFetch<CursorListResponse<PortalBanner>>('/portal/banners?limit=50');
  return response.data;
}

export async function createPortalBanner(body: CreatePortalBannerRequest): Promise<PortalBanner> {
  return apiPost<PortalBanner>('/portal/banners', body);
}

export async function updatePortalBanner(
  id: string,
  body: UpdatePortalBannerRequest,
): Promise<PortalBanner> {
  return apiPatch<PortalBanner>(`/portal/banners/${id}`, {
    body: JSON.stringify(body),
  });
}

export async function deletePortalBanner(id: string): Promise<void> {
  await apiDelete(`/portal/banners/${id}`);
}

export async function fetchPortalPromotions(): Promise<PortalPromotion[]> {
  const response = await apiFetch<CursorListResponse<PortalPromotion>>(
    '/portal/promotions?limit=50',
  );
  return response.data;
}

export async function createPortalPromotion(
  body: CreatePortalPromotionRequest,
): Promise<PortalPromotion> {
  return apiPost<PortalPromotion>('/portal/promotions', body);
}

export async function updatePortalPromotion(
  id: string,
  body: UpdatePortalPromotionRequest,
): Promise<PortalPromotion> {
  return apiPatch<PortalPromotion>(`/portal/promotions/${id}`, {
    body: JSON.stringify(body),
  });
}

export async function deletePortalPromotion(id: string): Promise<void> {
  await apiDelete(`/portal/promotions/${id}`);
}

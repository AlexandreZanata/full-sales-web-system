import { apiFetch } from '@/lib/api/client';
import { getAccessToken } from '@/lib/auth/tokens';
import type { CursorListResponse } from '@/lib/api/types';

export type PortalPromotion = {
  id: string;
  headline: string;
  discountText: string;
  imageUrl?: string;
  categorySlug?: string;
  linkUrl?: string;
  background: 'yellow' | 'green';
};

const DEMO_PROMOTIONS: PortalPromotion[] = [
  {
    id: 'demo-promo-1',
    headline: 'Tasty Burger',
    discountText: '30% OFF',
    categorySlug: 'snacks',
    background: 'yellow',
  },
  {
    id: 'demo-promo-2',
    headline: 'Fresh Salad',
    discountText: '15% OFF',
    categorySlug: 'bebidas',
    background: 'green',
  },
];

function promotionsPath(limit: number): { path: string; init?: { skipAuth: true } } {
  const query = new URLSearchParams({ limit: String(limit) });
  const hasSession = Boolean(getAccessToken());
  return hasSession
    ? { path: `/portal/promotions?${query}` }
    : { path: `/public/promotions?${query}`, init: { skipAuth: true } };
}

export async function fetchPortalPromotions(limit = 4): Promise<PortalPromotion[]> {
  try {
    const { path, init } = promotionsPath(limit);
    const response = await apiFetch<CursorListResponse<PortalPromotion>>(path, init);
    if (response.data.length > 0) {
      return response.data;
    }
  } catch {
    // MVP: API lands in Phase 71N — fall through to demo promos.
  }

  return DEMO_PROMOTIONS.slice(0, limit);
}

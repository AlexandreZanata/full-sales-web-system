import { apiFetch } from '@/lib/api/client';
import { getAccessToken } from '@/lib/auth/tokens';
import type { CursorListResponse, PortalProduct } from '@/lib/api/types';

import { fetchPortalProducts } from '@/lib/api/portal';

function popularProductsPath(limit: number): { path: string; init?: { skipAuth: true } } {
  const query = new URLSearchParams({ limit: String(limit) });
  const hasSession = Boolean(getAccessToken());
  return hasSession
    ? { path: `/portal/products/popular?${query}` }
    : { path: `/public/products/popular?${query}`, init: { skipAuth: true } };
}

export async function fetchPortalPopularProducts(limit = 12): Promise<PortalProduct[]> {
  try {
    const { path, init } = popularProductsPath(limit);
    const response = await apiFetch<CursorListResponse<PortalProduct>>(path, init);
    if (response.data.length > 0) {
      return response.data;
    }
  } catch {
    // MVP: API lands in Phase 71N — fall through to catalog sample.
  }

  try {
    const fallback = await fetchPortalProducts({ limit });
    return fallback.data.slice(0, limit);
  } catch {
    return [];
  }
}

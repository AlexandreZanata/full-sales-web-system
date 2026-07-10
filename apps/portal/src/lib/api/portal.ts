import { ApiError, apiDelete, apiFetch, apiPost, apiPut } from '@/lib/api/client';
import { fetchSettings } from '@/lib/api/settings';
import { getAccessToken } from '@/lib/auth/tokens';
import type {
  CreatePortalOrderRequest,
  CursorListResponse,
  PortalCategory,
  PortalCategoryWithProducts,
  PortalOrderDetail,
  PortalOrderSummary,
  PortalProduct,
  PortalProductDetail,
} from '@/lib/api/types';

export type PortalBanner = {
  id: string;
  imageUrl: string;
  linkUrl?: string;
  altText?: string;
};

export type PortalProductsParams = {
  limit?: number;
  cursor?: string;
  categorySlug?: string;
  search?: string;
};

export type PortalOrdersParams = {
  limit?: number;
  cursor?: string;
  status?: string;
};

export type PortalCategoryBySlugParams = {
  limit?: number;
  cursor?: string;
};

function portalAuthPath(
  portalPath: string,
  publicPath: string,
): {
  path: string;
  init?: { skipAuth: true };
} {
  const hasSession = Boolean(getAccessToken());
  return hasSession ? { path: portalPath } : { path: publicPath, init: { skipAuth: true } };
}

function buildProductListQuery(params: PortalProductsParams): string {
  const query = new URLSearchParams({ limit: String(params.limit ?? 50) });
  if (params.cursor) {
    query.set('cursor', params.cursor);
  }
  if (params.categorySlug) {
    query.set('filter[category_slug]', params.categorySlug);
  }
  return query.toString();
}

export async function fetchPortalCategories(): Promise<PortalCategory[]> {
  const { path, init } = portalAuthPath(
    '/portal/categories?limit=100',
    '/public/categories?limit=100',
  );
  const response = await apiFetch<CursorListResponse<PortalCategory>>(path, init);
  return response.data;
}

export async function fetchPortalCategoryBySlug(
  slug: string,
  params: PortalCategoryBySlugParams = {},
): Promise<PortalCategoryWithProducts> {
  const query = new URLSearchParams({ limit: String(params.limit ?? 50) });
  if (params.cursor) {
    query.set('cursor', params.cursor);
  }
  const { path, init } = portalAuthPath(
    `/portal/categories/${encodeURIComponent(slug)}?${query}`,
    `/public/categories/${encodeURIComponent(slug)}?${query}`,
  );
  return apiFetch<PortalCategoryWithProducts>(path, init);
}

export async function fetchPortalProductById(id: string): Promise<PortalProductDetail | null> {
  const { path, init } = portalAuthPath(
    `/portal/products/${encodeURIComponent(id)}`,
    `/public/products/${encodeURIComponent(id)}`,
  );

  try {
    return await apiFetch<PortalProductDetail>(path, init);
  } catch (error) {
    if (error instanceof ApiError && error.status === 404) {
      return null;
    }
    throw error;
  }
}

export async function fetchPortalProducts(
  params: PortalProductsParams = {},
): Promise<CursorListResponse<PortalProduct>> {
  const query = buildProductListQuery(params);
  const hasSession = Boolean(getAccessToken());
  const path = hasSession ? `/portal/products?${query}` : `/public/products?${query}`;
  const response = await apiFetch<CursorListResponse<PortalProduct>>(
    path,
    hasSession ? undefined : { skipAuth: true },
  );
  if (!params.search?.trim()) {
    return response;
  }
  const term = params.search.trim().toLowerCase();
  const filtered = response.data.filter(
    (product) =>
      product.name.toLowerCase().includes(term) || product.sku.toLowerCase().includes(term),
  );
  return { ...response, data: filtered };
}

export async function fetchPortalOrders(
  params: PortalOrdersParams = {},
): Promise<CursorListResponse<PortalOrderSummary>> {
  const query = new URLSearchParams({ limit: String(params.limit ?? 20) });
  if (params.cursor) {
    query.set('cursor', params.cursor);
  }
  if (params.status) {
    query.set('filter[status]', params.status);
  }
  return apiFetch<CursorListResponse<PortalOrderSummary>>(`/portal/orders?${query}`);
}

export async function fetchPortalOrder(id: string): Promise<PortalOrderDetail> {
  return apiFetch<PortalOrderDetail>(`/portal/orders/${id}`);
}

export async function createPortalOrder(
  body: CreatePortalOrderRequest,
): Promise<PortalOrderDetail> {
  return apiPost<PortalOrderDetail>('/portal/orders', body);
}

export async function updatePortalOrder(
  id: string,
  body: CreatePortalOrderRequest,
): Promise<PortalOrderDetail> {
  return apiPut<PortalOrderDetail>(`/portal/orders/${id}`, body);
}

export async function deletePortalOrder(id: string): Promise<void> {
  await apiDelete(`/portal/orders/${id}`);
}

const DEMO_HERO_BANNERS: PortalBanner[] = [
  {
    id: 'demo-hero-1',
    imageUrl: '/demo/hero-banner.svg',
    altText: 'Welcome',
  },
];

export async function fetchPortalBanners(placement = 'hero'): Promise<PortalBanner[]> {
  try {
    const query = new URLSearchParams({ placement, limit: '10' });
    const { path, init } = portalAuthPath(`/portal/banners?${query}`, `/public/banners?${query}`);
    const response = await apiFetch<CursorListResponse<PortalBanner>>(path, init);
    if (response.data.length > 0) {
      return response.data;
    }
  } catch {
    // MVP: API lands in Phase 71N — fall through to settings/demo banners.
  }

  try {
    const settings = await fetchSettings();
    if (settings.heroBanners?.length) {
      return settings.heroBanners;
    }
  } catch {
    // Keep demo asset when settings are unavailable.
  }

  return DEMO_HERO_BANNERS;
}

export async function submitPortalOrder(id: string): Promise<PortalOrderDetail> {
  return apiPost<PortalOrderDetail>(`/portal/orders/${id}/submit`, {});
}

import { apiDelete, apiFetch, apiPost, apiPut } from '@/lib/api/client';
import { getAccessToken } from '@/lib/auth/tokens';
import type {
  CreatePortalOrderRequest,
  PaginatedResponse,
  PortalCategory,
  PortalCategoryWithProducts,
  PortalOrderDetail,
  PortalOrderSummary,
  PortalProduct,
} from '@/lib/api/types';

export type PortalProductsParams = {
  page?: number;
  pageSize?: number;
  category?: string;
  search?: string;
};

export type PortalOrdersParams = {
  page?: number;
  pageSize?: number;
  status?: string;
};

export type PortalCategoryBySlugParams = {
  page?: number;
  pageSize?: number;
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

export async function fetchPortalCategories(): Promise<PortalCategory[]> {
  const { path, init } = portalAuthPath('/portal/categories', '/public/categories');
  return apiFetch<PortalCategory[]>(path, init);
}

export async function fetchPortalCategoryBySlug(
  slug: string,
  params: PortalCategoryBySlugParams = {},
): Promise<PortalCategoryWithProducts> {
  const query = new URLSearchParams({
    page: String(params.page ?? 1),
    pageSize: String(params.pageSize ?? 50),
  });
  const { path, init } = portalAuthPath(
    `/portal/categories/${encodeURIComponent(slug)}?${query}`,
    `/public/categories/${encodeURIComponent(slug)}?${query}`,
  );
  return apiFetch<PortalCategoryWithProducts>(path, init);
}

export async function fetchPortalProductById(
  id: string,
  categorySlug?: string,
): Promise<PortalProduct | null> {
  if (categorySlug) {
    const category = await fetchPortalCategoryBySlug(categorySlug);
    return category.products.find((product) => product.id === id) ?? null;
  }

  const categories = await fetchPortalCategories();
  for (const category of categories) {
    if (!category.active) {
      continue;
    }
    const data = await fetchPortalCategoryBySlug(category.slug);
    const product = data.products.find((item) => item.id === id);
    if (product) {
      return product;
    }
  }

  return null;
}

export async function fetchPortalProducts(
  params: PortalProductsParams = {},
): Promise<PaginatedResponse<PortalProduct>> {
  const query = new URLSearchParams({
    page: String(params.page ?? 1),
    pageSize: String(params.pageSize ?? 50),
  });
  if (params.category) {
    query.set('category', params.category);
  }
  const hasSession = Boolean(getAccessToken());
  const path = hasSession ? `/portal/products?${query}` : `/public/products?${query}`;
  const response = await apiFetch<PaginatedResponse<PortalProduct>>(
    path,
    hasSession ? undefined : { skipAuth: true },
  );
  if (!params.search?.trim()) {
    return response;
  }
  const term = params.search.trim().toLowerCase();
  return {
    ...response,
    items: response.items.filter(
      (product) =>
        product.name.toLowerCase().includes(term) || product.sku.toLowerCase().includes(term),
    ),
    total: response.items.filter(
      (product) =>
        product.name.toLowerCase().includes(term) || product.sku.toLowerCase().includes(term),
    ).length,
  };
}

export async function fetchPortalOrders(
  params: PortalOrdersParams = {},
): Promise<PaginatedResponse<PortalOrderSummary>> {
  const query = new URLSearchParams({
    page: String(params.page ?? 1),
    pageSize: String(params.pageSize ?? 20),
  });
  if (params.status) {
    query.set('status', params.status);
  }
  return apiFetch<PaginatedResponse<PortalOrderSummary>>(`/portal/orders?${query}`);
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

export async function submitPortalOrder(id: string): Promise<PortalOrderDetail> {
  return apiPost<PortalOrderDetail>(`/portal/orders/${id}/submit`, {});
}

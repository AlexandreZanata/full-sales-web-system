import { apiDelete, apiFetch, apiPost, apiPut } from '@/lib/api/client';
import type {
  CreatePortalOrderRequest,
  PaginatedResponse,
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
  const response = await apiFetch<PaginatedResponse<PortalProduct>>(`/portal/products?${query}`);
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

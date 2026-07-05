import { apiFetch, apiPost } from '@/lib/api/client';
import type {
  PaginatedResponse,
  ProductStockOverview,
  RecordMovementRequest,
  StockBalance,
  StockMovement,
} from '@/lib/api/types';

export async function fetchStockBalance(productId: string): Promise<StockBalance> {
  return apiFetch<StockBalance>(`/inventory/products/${productId}/balance`);
}

export type StockOverviewParams = {
  page: number;
  pageSize: number;
  search?: string;
};

export async function fetchStockOverview(
  params: StockOverviewParams,
): Promise<PaginatedResponse<ProductStockOverview>> {
  const query = new URLSearchParams({
    page: String(params.page),
    pageSize: String(params.pageSize),
  });
  if (params.search) {
    query.set('search', params.search);
  }
  return apiFetch<PaginatedResponse<ProductStockOverview>>(`/inventory/balances?${query}`);
}

export async function recordMovement(body: RecordMovementRequest): Promise<StockMovement> {
  return apiPost<StockMovement>('/inventory/movements', body);
}

export type MovementsListParams = {
  productId: string;
  page: number;
  pageSize: number;
};

export async function fetchMovements(
  params: MovementsListParams,
): Promise<PaginatedResponse<StockMovement>> {
  const query = new URLSearchParams({
    page: String(params.page),
    pageSize: String(params.pageSize),
  });
  return apiFetch<PaginatedResponse<StockMovement>>(
    `/inventory/products/${params.productId}/movements?${query}`,
  );
}

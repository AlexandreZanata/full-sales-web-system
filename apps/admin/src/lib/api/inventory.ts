import { apiFetch, apiPost } from '@/lib/api/client';
import type { CursorListParams, CursorListResponse } from '@/lib/cursorPagination';
import type {
  ProductStockOverview,
  RecordMovementRequest,
  StockBalance,
  StockMovement,
} from '@/lib/api/types';

export async function fetchStockBalance(productId: string): Promise<StockBalance> {
  return apiFetch<StockBalance>(`/inventory/products/${productId}/balance`);
}

export type StockOverviewParams = CursorListParams & {
  search?: string;
};

export async function fetchStockOverview(
  params: StockOverviewParams,
): Promise<CursorListResponse<ProductStockOverview>> {
  const query = new URLSearchParams();
  query.set('limit', String(params.limit ?? 20));
  if (params.cursor) {
    query.set('cursor', params.cursor);
  }
  if (params.search) {
    query.set('filter[sku][like]', params.search);
  }
  return apiFetch<CursorListResponse<ProductStockOverview>>(`/inventory/balances?${query}`);
}

export async function recordMovement(body: RecordMovementRequest): Promise<StockMovement> {
  return apiPost<StockMovement>('/inventory/movements', body);
}

export type MovementsListParams = {
  productId: string;
} & CursorListParams;

export async function fetchMovements(
  params: MovementsListParams,
): Promise<CursorListResponse<StockMovement>> {
  const query = new URLSearchParams();
  query.set('limit', String(params.limit ?? 20));
  if (params.cursor) {
    query.set('cursor', params.cursor);
  }
  return apiFetch<CursorListResponse<StockMovement>>(
    `/inventory/products/${params.productId}/movements?${query}`,
  );
}

import { apiFetch, apiPost } from '@/lib/api/client';
import { dateFilterToIso } from '@/lib/api/orders';
import { type CursorListParams, type CursorListResponse } from '@/lib/cursorPagination';
import type { CreateSaleRequest, SaleDetail, SaleSummary } from '@/lib/api/types';
import type { SaleStatusFilter } from '@/lib/sales/constants';

export type SalesListParams = CursorListParams & {
  status?: SaleStatusFilter;
  commerceId?: string;
  driverId?: string;
  from?: string;
  to?: string;
};

function buildSalesQuery(params: SalesListParams): URLSearchParams {
  const query = new URLSearchParams();
  query.set('limit', String(params.limit ?? 20));
  if (params.cursor) {
    query.set('cursor', params.cursor);
  }
  if (params.status) {
    query.set('filter[status]', params.status);
  }
  if (params.commerceId) {
    query.set('filter[commerce_id]', params.commerceId);
  }
  if (params.driverId) {
    query.set('filter[driver_id]', params.driverId);
  }
  if (params.from) {
    query.set('filter[created_at][gte]', params.from);
  }
  if (params.to) {
    query.set('filter[created_at][lte]', params.to);
  }
  return query;
}

export async function fetchSales(
  params: SalesListParams,
): Promise<CursorListResponse<SaleSummary>> {
  const query = buildSalesQuery(params);
  return apiFetch<CursorListResponse<SaleSummary>>(`/sales?${query}`);
}

export async function fetchSale(id: string): Promise<SaleDetail> {
  return apiFetch<SaleDetail>(`/sales/${id}`);
}

export async function confirmSale(id: string): Promise<SaleDetail> {
  return apiPost<SaleDetail>(`/sales/${id}/confirm`, {});
}

export async function cancelSale(id: string): Promise<SaleDetail> {
  return apiPost<SaleDetail>(`/sales/${id}/cancel`, {});
}

export async function createSale(
  body: CreateSaleRequest,
  idempotencyKey: string,
): Promise<SaleDetail> {
  return apiPost<SaleDetail>('/sales', body, {
    headers: { 'Idempotency-Key': idempotencyKey },
  });
}

export { dateFilterToIso };

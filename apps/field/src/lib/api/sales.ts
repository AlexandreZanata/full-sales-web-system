import { apiFetch, apiPost } from '@/lib/api/client';
import { type CursorListParams, type CursorListResponse } from '@/lib/cursorPagination';
import type { CreateSaleRequest, SaleDetail, SaleSummary } from '@/lib/api/types';

export type SalesListParams = CursorListParams;

export async function fetchSales(
  params: SalesListParams = {},
): Promise<CursorListResponse<SaleSummary>> {
  const query = new URLSearchParams({
    limit: String(params.limit ?? 20),
  });
  if (params.cursor) {
    query.set('cursor', params.cursor);
  }
  return apiFetch<CursorListResponse<SaleSummary>>(`/sales?${query}`);
}

export async function fetchSale(id: string): Promise<SaleDetail> {
  return apiFetch<SaleDetail>(`/sales/${id}`);
}

export async function createSale(
  body: CreateSaleRequest,
  idempotencyKey: string,
): Promise<SaleDetail> {
  return apiPost<SaleDetail>('/sales', body, {
    headers: { 'Idempotency-Key': idempotencyKey },
  });
}

export async function confirmSale(id: string): Promise<SaleDetail> {
  return apiPost<SaleDetail>(`/sales/${id}/confirm`, {});
}

export async function cancelSale(id: string): Promise<SaleDetail> {
  return apiPost<SaleDetail>(`/sales/${id}/cancel`, {});
}

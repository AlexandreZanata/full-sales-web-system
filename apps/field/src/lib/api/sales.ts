import { apiFetch, apiPost } from '@/lib/api/client';
import type {
  CreateSaleRequest,
  PaginatedResponse,
  SaleDetail,
  SaleSummary,
} from '@/lib/api/types';

export async function fetchSales(
  params: {
    page?: number;
    pageSize?: number;
  } = {},
): Promise<PaginatedResponse<SaleSummary>> {
  const query = new URLSearchParams({
    page: String(params.page ?? 1),
    pageSize: String(params.pageSize ?? 20),
  });
  return apiFetch<PaginatedResponse<SaleSummary>>(`/sales?${query}`);
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

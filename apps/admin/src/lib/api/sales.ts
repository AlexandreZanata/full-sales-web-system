import { apiFetch, apiPost } from '@/lib/api/client';
import { dateFilterToIso } from '@/lib/api/orders';
import type {
  CreateSaleRequest,
  PaginatedResponse,
  SaleDetail,
  SaleSummary,
} from '@/lib/api/types';
import type { SaleStatusFilter } from '@/lib/sales/constants';

export type SalesListParams = {
  page: number;
  pageSize: number;
  status?: SaleStatusFilter;
  commerceId?: string;
  driverId?: string;
  from?: string;
  to?: string;
};

function buildSalesQuery(params: SalesListParams): URLSearchParams {
  const query = new URLSearchParams({
    page: String(params.page),
    pageSize: String(params.pageSize),
  });
  if (params.status) {
    query.set('status', params.status);
  }
  if (params.commerceId) {
    query.set('commerceId', params.commerceId);
  }
  if (params.driverId) {
    query.set('driverId', params.driverId);
  }
  if (params.from) {
    query.set('from', params.from);
  }
  if (params.to) {
    query.set('to', params.to);
  }
  return query;
}

export async function fetchSales(params: SalesListParams): Promise<PaginatedResponse<SaleSummary>> {
  const query = buildSalesQuery(params);
  return apiFetch<PaginatedResponse<SaleSummary>>(`/sales?${query}`);
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

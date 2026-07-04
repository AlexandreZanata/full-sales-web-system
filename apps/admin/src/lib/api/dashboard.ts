import { apiFetch } from '@/lib/api/client';
import type { PaginatedResponse, SaleSummary } from '@/lib/api/types';

export async function fetchPendingOrdersCount(): Promise<number> {
  const data = await apiFetch<PaginatedResponse<{ id: string }>>(
    '/orders?status=PendingApproval&pageSize=1',
  );
  return data.total;
}

export async function fetchWaitingDeliveriesCount(): Promise<number> {
  const data = await apiFetch<PaginatedResponse<{ id: string }>>(
    '/deliveries?status=Waiting&pageSize=1',
  );
  return data.total;
}

export async function fetchRecentSales(pageSize = 5): Promise<SaleSummary[]> {
  const data = await apiFetch<PaginatedResponse<SaleSummary>>(
    `/sales?pageSize=${String(pageSize)}`,
  );
  return data.items;
}

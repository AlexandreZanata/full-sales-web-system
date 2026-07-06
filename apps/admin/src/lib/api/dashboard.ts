import { apiFetch } from '@/lib/api/client';
import { fetchDeliveries } from '@/lib/api/deliveries';
import { fetchOrders } from '@/lib/api/orders';
import { fetchAllCursorPages } from '@/lib/cursorPagination';
import type { SaleSummary } from '@/lib/api/types';

export async function fetchPendingOrdersCount(): Promise<number> {
  const items = await fetchAllCursorPages(
    async (cursor) => fetchOrders({ limit: 100, cursor, status: 'PendingApproval' }),
    100,
  );
  return items.length;
}

export async function fetchWaitingDeliveriesCount(): Promise<number> {
  const items = await fetchAllCursorPages(
    async (cursor) => fetchDeliveries({ limit: 100, cursor, status: 'Waiting' }),
    100,
  );
  return items.length;
}

export async function fetchRecentSales(pageSize = 5): Promise<SaleSummary[]> {
  const data = await apiFetch<{ data: SaleSummary[] }>(`/sales?limit=${String(pageSize)}`);
  return data.data;
}

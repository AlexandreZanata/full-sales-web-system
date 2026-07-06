import { apiFetch, apiPost } from '@/lib/api/client';
import {
  type CursorListParams,
  type CursorListResponse,
  fetchAllCursorPages,
} from '@/lib/cursorPagination';
import type { DeliverySummary, OrderDetail, OrderSummary } from '@/lib/api/types';
import type { OrderStatusFilter } from '@/lib/orders/constants';

export type OrdersListParams = CursorListParams & {
  status?: OrderStatusFilter;
  commerceId?: string;
  from?: string;
  to?: string;
};

function buildOrdersQuery(params: OrdersListParams): string {
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
  if (params.from) {
    query.set('filter[created_at][gte]', params.from);
  }
  if (params.to) {
    query.set('filter[created_at][lte]', params.to);
  }
  return query.toString();
}

export async function fetchOrders(
  params: OrdersListParams,
): Promise<CursorListResponse<OrderSummary>> {
  return apiFetch<CursorListResponse<OrderSummary>>(`/orders?${buildOrdersQuery(params)}`);
}

export async function fetchAllOrders(
  params: Omit<OrdersListParams, 'cursor' | 'limit'> = {},
): Promise<OrderSummary[]> {
  return fetchAllCursorPages(async (cursor) =>
    fetchOrders({ ...params, limit: 100, cursor }),
  );
}

export async function fetchOrder(id: string): Promise<OrderDetail> {
  return apiFetch<OrderDetail>(`/orders/${id}`);
}

export async function approveOrder(id: string): Promise<OrderDetail> {
  return apiPost<OrderDetail>(`/orders/${id}/approve`, {});
}

export async function rejectOrder(id: string, reason: string): Promise<OrderDetail> {
  return apiPost<OrderDetail>(`/orders/${id}/reject`, { reason });
}

export async function cancelOrder(id: string): Promise<OrderDetail> {
  return apiPost<OrderDetail>(`/orders/${id}/cancel`, {});
}

export async function startPicking(id: string): Promise<OrderDetail> {
  return apiPost<OrderDetail>(`/orders/${id}/start-picking`, {});
}

export async function assignDelivery(orderId: string, driverId: string): Promise<DeliverySummary> {
  return apiPost<DeliverySummary>(`/orders/${orderId}/delivery`, { driverId });
}

/** Date-only filter → ISO start/end of day (UTC). */
export function dateFilterToIso(date: string, boundary: 'start' | 'end'): string {
  if (boundary === 'start') {
    return new Date(`${date}T00:00:00.000Z`).toISOString();
  }
  return new Date(`${date}T23:59:59.999Z`).toISOString();
}

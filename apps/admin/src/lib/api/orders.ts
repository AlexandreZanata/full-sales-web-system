import { apiFetch, apiPost } from '@/lib/api/client';
import type {
  DeliverySummary,
  OrderDetail,
  OrderSummary,
  PaginatedResponse,
} from '@/lib/api/types';
import type { OrderStatusFilter } from '@/lib/orders/constants';

export type OrdersListParams = {
  page: number;
  pageSize: number;
  status?: OrderStatusFilter;
  commerceId?: string;
  from?: string;
  to?: string;
};

function buildOrdersQuery(params: OrdersListParams): URLSearchParams {
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
  if (params.from) {
    query.set('from', params.from);
  }
  if (params.to) {
    query.set('to', params.to);
  }
  return query;
}

export async function fetchOrders(
  params: OrdersListParams,
): Promise<PaginatedResponse<OrderSummary>> {
  const query = buildOrdersQuery(params);
  return apiFetch<PaginatedResponse<OrderSummary>>(`/orders?${query}`);
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

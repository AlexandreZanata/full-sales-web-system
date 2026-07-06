import { apiFetch } from '@/lib/api/client';
import { type CursorListParams, type CursorListResponse } from '@/lib/cursorPagination';
import type { DeliveryDetail } from '@/lib/api/types';
import type { DeliveryStatusFilter } from '@/lib/deliveries/constants';

export type DeliveriesListParams = CursorListParams & {
  status?: DeliveryStatusFilter;
  from?: string;
  to?: string;
};

function buildDeliveriesQuery(params: DeliveriesListParams): string {
  const query = new URLSearchParams();
  query.set('limit', String(params.limit ?? 20));
  if (params.cursor) {
    query.set('cursor', params.cursor);
  }
  if (params.status) {
    query.set('filter[status]', params.status);
  }
  if (params.from) {
    query.set('filter[created_at][gte]', params.from);
  }
  if (params.to) {
    query.set('filter[created_at][lte]', params.to);
  }
  return query.toString();
}

export async function fetchDeliveries(
  params: DeliveriesListParams,
): Promise<CursorListResponse<DeliveryDetail>> {
  return apiFetch<CursorListResponse<DeliveryDetail>>(
    `/deliveries?${buildDeliveriesQuery(params)}`,
  );
}

export async function fetchDelivery(id: string): Promise<DeliveryDetail> {
  return apiFetch<DeliveryDetail>(`/deliveries/${id}`);
}

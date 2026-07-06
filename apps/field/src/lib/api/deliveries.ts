import { apiFetch, apiPost } from '@/lib/api/client';
import { type CursorListParams, type CursorListResponse } from '@/lib/cursorPagination';
import type { DeliveryDetail } from '@/lib/api/types';
import type { DeliveryStatusFilter } from '@/lib/deliveries/constants';

export type DeliveriesListParams = CursorListParams & {
  status?: DeliveryStatusFilter;
};

export async function fetchDeliveries(
  params: DeliveriesListParams = {},
): Promise<CursorListResponse<DeliveryDetail>> {
  const query = new URLSearchParams({
    limit: String(params.limit ?? 20),
  });
  if (params.cursor) {
    query.set('cursor', params.cursor);
  }
  if (params.status) {
    query.set('filter[status]', params.status);
  }
  return apiFetch<CursorListResponse<DeliveryDetail>>(`/deliveries?${query}`);
}

export async function fetchDelivery(id: string): Promise<DeliveryDetail> {
  return apiFetch<DeliveryDetail>(`/deliveries/${id}`);
}

export async function startDeliveryTransit(id: string): Promise<DeliveryDetail> {
  return apiPost<DeliveryDetail>(`/deliveries/${id}/start-transit`, {});
}

export type ConfirmDeliveryItem = {
  orderItemId: string;
  quantityDelivered: number;
};

export type ConfirmDeliveryRequest = {
  proofFileId: string;
  items: ConfirmDeliveryItem[];
  receivedByName?: string;
};

export async function confirmDelivery(
  id: string,
  body: ConfirmDeliveryRequest,
): Promise<DeliveryDetail> {
  return apiPost<DeliveryDetail>(`/deliveries/${id}/confirm`, body);
}

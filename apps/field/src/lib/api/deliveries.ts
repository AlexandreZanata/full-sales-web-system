import { apiFetch, apiPost } from '@/lib/api/client';
import type { DeliveryDetail, PaginatedResponse } from '@/lib/api/types';
import type { DeliveryStatusFilter } from '@/lib/deliveries/constants';

export type DeliveriesListParams = {
  page?: number;
  pageSize?: number;
  status?: DeliveryStatusFilter;
};

export async function fetchDeliveries(
  params: DeliveriesListParams = {},
): Promise<PaginatedResponse<DeliveryDetail>> {
  const query = new URLSearchParams({
    page: String(params.page ?? 1),
    pageSize: String(params.pageSize ?? 20),
  });
  if (params.status) {
    query.set('status', params.status);
  }
  return apiFetch<PaginatedResponse<DeliveryDetail>>(`/deliveries?${query}`);
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

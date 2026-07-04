import { apiFetch } from '@/lib/api/client';
import type { DeliveryDetail, PaginatedResponse } from '@/lib/api/types';
import type { DeliveryStatusFilter } from '@/lib/deliveries/constants';

export type DeliveriesListParams = {
  page: number;
  pageSize: number;
  status?: DeliveryStatusFilter;
};

export async function fetchDeliveries(
  params: DeliveriesListParams,
): Promise<PaginatedResponse<DeliveryDetail>> {
  const query = new URLSearchParams({
    page: String(params.page),
    pageSize: String(params.pageSize),
  });
  if (params.status) {
    query.set('status', params.status);
  }
  return apiFetch<PaginatedResponse<DeliveryDetail>>(`/deliveries?${query}`);
}

export async function fetchDelivery(id: string): Promise<DeliveryDetail> {
  return apiFetch<DeliveryDetail>(`/deliveries/${id}`);
}

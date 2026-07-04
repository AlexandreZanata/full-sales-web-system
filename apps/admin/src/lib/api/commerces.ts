import { apiFetch, apiPatch, apiPost, apiPut } from '@/lib/api/client';
import type {
  Commerce,
  CommerceAddress,
  CommerceAddressRequest,
  CommerceSummary,
  CreateCommerceRequest,
  PaginatedResponse,
  UpdateCommerceAddressRequest,
} from '@/lib/api/types';
import type { ActiveFilter } from '@/lib/commerces/constants';

export type CommercesListParams = {
  page: number;
  pageSize: number;
  active?: ActiveFilter;
};

function buildActiveQuery(active?: ActiveFilter): string {
  if (active === 'true' || active === 'false') {
    return active;
  }
  return '';
}

export async function fetchCommerces(
  params: CommercesListParams,
): Promise<PaginatedResponse<CommerceSummary>> {
  const query = new URLSearchParams({
    page: String(params.page),
    pageSize: String(params.pageSize),
  });
  const active = buildActiveQuery(params.active);
  if (active) {
    query.set('active', active);
  }
  return apiFetch<PaginatedResponse<CommerceSummary>>(`/commerces?${query}`);
}

export async function fetchCommercesForPicker(): Promise<CommerceSummary[]> {
  const data = await fetchCommerces({ page: 1, pageSize: 50, active: 'true' });
  return data.items;
}

export async function fetchCommerce(id: string): Promise<Commerce> {
  return apiFetch<Commerce>(`/commerces/${id}`);
}

export async function createCommerce(body: CreateCommerceRequest): Promise<Commerce> {
  return apiPost<Commerce>('/commerces', body);
}

export async function deactivateCommerce(id: string): Promise<Commerce> {
  return apiPatch<Commerce>(`/commerces/${id}/deactivate`);
}

export async function fetchCommerceAddresses(commerceId: string): Promise<CommerceAddress[]> {
  return apiFetch<CommerceAddress[]>(`/commerces/${commerceId}/addresses`);
}

export async function createCommerceAddress(
  commerceId: string,
  body: CommerceAddressRequest,
): Promise<CommerceAddress> {
  return apiPost<CommerceAddress>(`/commerces/${commerceId}/addresses`, body);
}

export async function updateCommerceAddress(
  commerceId: string,
  addressId: string,
  body: UpdateCommerceAddressRequest,
): Promise<CommerceAddress> {
  return apiPatch<CommerceAddress>(`/commerces/${commerceId}/addresses/${addressId}`, {
    body: JSON.stringify(body),
  });
}

export async function updateCommerceLogo(commerceId: string, fileId: string): Promise<Commerce> {
  return apiPut<Commerce>(`/commerces/${commerceId}/logo`, { fileId });
}

import { apiFetch, apiPatch, apiPost, apiPut } from '@/lib/api/client';
import {
  type CursorListParams,
  type CursorListResponse,
  fetchAllCursorPages,
} from '@/lib/cursorPagination';
import type {
  Commerce,
  CommerceAddress,
  CommerceAddressRequest,
  CommerceSummary,
  CreateCommerceRequest,
  UpdateCommerceAddressRequest,
} from '@/lib/api/types';
import type { ActiveFilter } from '@/lib/commerces/constants';

export type CommercesListParams = CursorListParams & {
  active?: ActiveFilter;
};

function buildCommercesQuery(params: CommercesListParams): string {
  const query = new URLSearchParams();
  query.set('limit', String(params.limit ?? 20));
  if (params.cursor) {
    query.set('cursor', params.cursor);
  }
  if (params.active === 'true' || params.active === 'false') {
    query.set('filter[active]', params.active);
  }
  return query.toString();
}

export async function fetchCommerces(
  params: CommercesListParams,
): Promise<CursorListResponse<CommerceSummary>> {
  return apiFetch<CursorListResponse<CommerceSummary>>(`/commerces?${buildCommercesQuery(params)}`);
}

export async function fetchCommercesForPicker(): Promise<CommerceSummary[]> {
  return fetchAllCursorPages(async (cursor) =>
    fetchCommerces({ limit: 100, cursor, active: 'true' }),
  );
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
  return fetchAllCursorPages(async (cursor) =>
    apiFetch<CursorListResponse<CommerceAddress>>(
      `/commerces/${commerceId}/addresses?limit=100${cursor ? `&cursor=${cursor}` : ''}`,
    ),
  );
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

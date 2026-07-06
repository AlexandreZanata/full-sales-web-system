import { apiFetch, apiPatch, apiPost } from '@/lib/api/client';
import type { CursorListParams, CursorListResponse } from '@/lib/cursorPagination';
import type {
  CommerceRegistration,
  PatchRegistrationRequest,
  RejectRegistrationRequest,
  RegistrationStatus,
} from '@/lib/api/types';

export type RegistrationsListParams = CursorListParams & {
  status?: RegistrationStatus;
};

function buildQuery(params: RegistrationsListParams): string {
  const query = new URLSearchParams();
  query.set('limit', String(params.limit ?? 20));
  if (params.cursor) {
    query.set('cursor', params.cursor);
  }
  if (params.status) {
    query.set('filter[status]', params.status);
  }
  return query.toString();
}

export async function fetchCommerceRegistrations(
  params: RegistrationsListParams,
): Promise<CursorListResponse<CommerceRegistration>> {
  return apiFetch<CursorListResponse<CommerceRegistration>>(
    `/commerces/registrations?${buildQuery(params)}`,
  );
}

export async function fetchCommerceRegistration(id: string): Promise<CommerceRegistration> {
  return apiFetch<CommerceRegistration>(`/commerces/registrations/${id}`);
}

export async function approveCommerceRegistration(id: string): Promise<CommerceRegistration> {
  return apiPost<CommerceRegistration>(`/commerces/registrations/${id}/approve`, {});
}

export async function rejectCommerceRegistration(
  id: string,
  body: RejectRegistrationRequest,
): Promise<CommerceRegistration> {
  return apiPost<CommerceRegistration>(`/commerces/registrations/${id}/reject`, body);
}

export async function patchCommerceRegistration(
  id: string,
  body: PatchRegistrationRequest,
): Promise<CommerceRegistration> {
  return apiPatch<CommerceRegistration>(`/commerces/registrations/${id}`, {
    body: JSON.stringify(body),
  });
}

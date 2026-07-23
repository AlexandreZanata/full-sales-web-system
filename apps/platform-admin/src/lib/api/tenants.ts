import { apiFetch, apiPatch, apiPost } from '@/lib/api/client';
import type { CursorListResponse } from '@/lib/cursorPagination';
import type { ProvisionTenantResponse, TenantDetail, TenantListItem } from '@/lib/api/types';

export type TenantsListParams = {
  limit?: number;
  cursor?: string;
  status?: string;
  planId?: string;
};

function buildQuery(params: TenantsListParams): string {
  const search = new URLSearchParams();
  if (params.limit) {
    search.set('limit', String(params.limit));
  }
  if (params.cursor) {
    search.set('cursor', params.cursor);
  }
  if (params.status) {
    search.set('filter[status]', params.status);
  }
  if (params.planId) {
    search.set('filter[plan_id]', params.planId);
  }
  const query = search.toString();
  return query ? `?${query}` : '';
}

export async function fetchTenants(
  params: TenantsListParams = {},
): Promise<CursorListResponse<TenantListItem>> {
  return apiFetch<CursorListResponse<TenantListItem>>(`/platform/tenants${buildQuery(params)}`);
}

export async function fetchTenant(id: string): Promise<TenantDetail> {
  return apiFetch<TenantDetail>(`/platform/tenants/${id}`);
}

export type CreateTenantBody = {
  legalName: string;
  displayName: string;
  planId: string;
  adminEmail: string;
  cnpj: string;
};

export async function createTenant(body: CreateTenantBody): Promise<ProvisionTenantResponse> {
  return apiPost<ProvisionTenantResponse>('/platform/tenants', body);
}

export type PatchTenantBody = {
  displayName?: string;
  planId?: string;
  graceExtendedUntil?: string;
  settings?: Record<string, unknown>;
};

export async function patchTenant(id: string, body: PatchTenantBody): Promise<TenantDetail> {
  return apiPatch<TenantDetail>(`/platform/tenants/${id}`, body);
}

export async function suspendTenant(id: string, reason: string): Promise<TenantDetail> {
  return apiPost<TenantDetail>(`/platform/tenants/${id}/suspend`, { reason });
}

export async function reactivateTenant(id: string): Promise<TenantDetail> {
  return apiPost<TenantDetail>(`/platform/tenants/${id}/reactivate`, {});
}

export async function offboardTenant(id: string): Promise<TenantDetail> {
  return apiPost<TenantDetail>(`/platform/tenants/${id}/offboard`, {});
}

export async function runDunningJob(): Promise<{ processed: string[] }> {
  return apiPost('/platform/jobs/dunning', {});
}

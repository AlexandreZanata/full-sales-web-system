import { apiFetch, apiPatch, apiPost } from '@/lib/api/client';
import type { CursorListResponse } from '@/lib/cursorPagination';
import type { PlatformUser } from '@/lib/api/types';

export type PlatformUsersParams = {
  limit?: number;
  cursor?: string;
  tenantId?: string;
  role?: string;
  active?: boolean;
  emailPrefix?: string;
};

function buildQuery(params: PlatformUsersParams): string {
  const search = new URLSearchParams();
  if (params.limit) {
    search.set('limit', String(params.limit));
  }
  if (params.cursor) {
    search.set('cursor', params.cursor);
  }
  if (params.tenantId) {
    search.set('filter[tenant_id]', params.tenantId);
  }
  if (params.role) {
    search.set('filter[role]', params.role);
  }
  if (params.active !== undefined) {
    search.set('filter[active]', String(params.active));
  }
  if (params.emailPrefix) {
    search.set('filter[email][prefix]', params.emailPrefix);
  }
  const query = search.toString();
  return query ? `?${query}` : '';
}

export async function fetchPlatformUsers(
  params: PlatformUsersParams = {},
): Promise<CursorListResponse<PlatformUser>> {
  return apiFetch<CursorListResponse<PlatformUser>>(`/platform/users${buildQuery(params)}`);
}

export async function fetchPlatformUser(id: string): Promise<PlatformUser> {
  return apiFetch<PlatformUser>(`/platform/users/${id}`);
}

export async function patchPlatformUser(id: string, body: { role: string }): Promise<PlatformUser> {
  return apiPatch<PlatformUser>(`/platform/users/${id}`, body);
}

export async function disablePlatformUser(id: string): Promise<{ active: false }> {
  return apiPost(`/platform/users/${id}/disable`, {});
}

export async function enablePlatformUser(id: string): Promise<{ active: true }> {
  return apiPost(`/platform/users/${id}/enable`, {});
}

export async function resetPlatformUserPassword(
  id: string,
): Promise<{ queued: boolean; temporaryPassword: string }> {
  return apiPost(`/platform/users/${id}/reset-password`, {});
}

export async function fetchTenantWorkforce(
  tenantId: string,
  params: { limit?: number; cursor?: string } = {},
): Promise<CursorListResponse<PlatformUser>> {
  const search = new URLSearchParams();
  if (params.limit) {
    search.set('limit', String(params.limit));
  }
  if (params.cursor) {
    search.set('cursor', params.cursor);
  }
  const query = search.toString();
  return apiFetch<CursorListResponse<PlatformUser>>(
    `/platform/tenants/${tenantId}/users${query ? `?${query}` : ''}`,
  );
}

export async function createPlatformTenantUser(
  tenantId: string,
  body: { name: string; email: string; role: string; commerceId?: string },
): Promise<{ user: PlatformUser; temporaryPassword: string }> {
  return apiPost(`/platform/tenants/${tenantId}/users`, body);
}

export async function fetchTenantStats(
  tenantId: string,
): Promise<import('@/lib/api/types').TenantStats> {
  return apiFetch(`/platform/tenants/${tenantId}/stats`);
}

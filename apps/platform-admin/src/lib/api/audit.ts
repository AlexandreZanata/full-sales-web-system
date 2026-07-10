import { apiFetch } from '@/lib/api/client';
import type { CursorListResponse } from '@/lib/cursorPagination';
import type { AuditEvent } from '@/lib/api/types';

export async function fetchPlatformAuditEvents(params: {
  limit?: number;
  cursor?: string;
  tenantId?: string;
  actorId?: string;
  action?: string;
  createdAtGte?: string;
  createdAtLte?: string;
}): Promise<CursorListResponse<AuditEvent>> {
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
  if (params.actorId) {
    search.set('filter[actor_id]', params.actorId);
  }
  if (params.action) {
    search.set('filter[action]', params.action);
  }
  if (params.createdAtGte) {
    search.set('filter[created_at][gte]', params.createdAtGte);
  }
  if (params.createdAtLte) {
    search.set('filter[created_at][lte]', params.createdAtLte);
  }
  const query = search.toString();
  return apiFetch<CursorListResponse<AuditEvent>>(
    `/platform/audit/events${query ? `?${query}` : ''}`,
  );
}

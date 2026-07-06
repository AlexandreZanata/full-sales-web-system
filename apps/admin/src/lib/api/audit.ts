import { apiFetch } from '@/lib/api/client';
import { type CursorListParams, type CursorListResponse } from '@/lib/cursorPagination';
import type { AuditEvent } from '@/lib/api/types';

export type AuditEventsListParams = CursorListParams & {
  actorId?: string;
  action?: string;
  from?: string;
  to?: string;
};

function buildAuditQuery(params: AuditEventsListParams): string {
  const query = new URLSearchParams();
  query.set('limit', String(params.limit ?? 20));
  if (params.cursor) {
    query.set('cursor', params.cursor);
  }
  if (params.actorId) {
    query.set('filter[actor_id]', params.actorId);
  }
  if (params.action) {
    query.set('filter[action]', params.action);
  }
  if (params.from) {
    query.set('filter[created_at][gte]', params.from);
  }
  if (params.to) {
    query.set('filter[created_at][lte]', params.to);
  }
  return query.toString();
}

export async function fetchAuditEvents(
  params: AuditEventsListParams,
): Promise<CursorListResponse<AuditEvent>> {
  return apiFetch<CursorListResponse<AuditEvent>>(`/audit/events?${buildAuditQuery(params)}`);
}

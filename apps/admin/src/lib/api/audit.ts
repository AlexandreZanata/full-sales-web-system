import { apiFetch } from '@/lib/api/client';
import type { AuditEvent, PaginatedResponse } from '@/lib/api/types';

export type AuditEventsListParams = {
  page: number;
  pageSize: number;
};

function buildAuditQuery(params: AuditEventsListParams): URLSearchParams {
  return new URLSearchParams({
    page: String(params.page),
    pageSize: String(params.pageSize),
  });
}

export async function fetchAuditEvents(
  params: AuditEventsListParams,
): Promise<PaginatedResponse<AuditEvent>> {
  const query = buildAuditQuery(params);
  return apiFetch<PaginatedResponse<AuditEvent>>(`/audit/events?${query}`);
}

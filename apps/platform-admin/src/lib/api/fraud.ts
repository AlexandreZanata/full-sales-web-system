import { apiFetch, apiPost } from '@/lib/api/client';
import type { CursorListResponse } from '@/lib/cursorPagination';
import type { FraudEvent } from '@/lib/api/types';

export async function fetchFraudEvents(params: {
  limit?: number;
  cursor?: string;
  status?: string;
}): Promise<CursorListResponse<FraudEvent>> {
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
  const query = search.toString();
  return apiFetch<CursorListResponse<FraudEvent>>(
    `/platform/fraud/events${query ? `?${query}` : ''}`,
  );
}

export async function resolveFraudEvent(
  id: string,
  body: { resolution: string; note?: string },
): Promise<FraudEvent> {
  return apiPost<FraudEvent>(`/platform/fraud/events/${id}/resolve`, body);
}

export async function addBlocklistEntry(body: {
  kind: string;
  value: string;
  reason?: string;
}): Promise<{ id: string }> {
  return apiPost('/platform/blocklist', body);
}

export async function deleteBlocklistEntry(id: string): Promise<void> {
  const { apiDelete } = await import('@/lib/api/client');
  return apiDelete(`/platform/blocklist/${id}`);
}

import { apiFetch, apiPatch, apiPost } from '@/lib/api/client';
import type { CursorListResponse } from '@/lib/cursorPagination';
import type { PlatformDomain } from '@/lib/api/types';

export async function fetchPlatformDomains(params: {
  limit?: number;
  cursor?: string;
  status?: string;
}): Promise<CursorListResponse<PlatformDomain>> {
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
  return apiFetch<CursorListResponse<PlatformDomain>>(
    `/platform/domains${query ? `?${query}` : ''}`,
  );
}

export async function forceVerifyDomain(id: string): Promise<PlatformDomain> {
  return apiPost<PlatformDomain>(`/platform/domains/${id}/force-verify`, {});
}

export async function patchPlatformDomain(
  id: string,
  body: { status?: string },
): Promise<PlatformDomain> {
  return apiPatch<PlatformDomain>(`/platform/domains/${id}`, body);
}

import { apiFetch } from '@/lib/api/client';
import { fetchAllCursorPages, type CursorListResponse } from '@/lib/cursorPagination';
import type { CommerceSummary } from '@/lib/api/types';

export async function fetchCommerces(search?: string): Promise<CommerceSummary[]> {
  const commerces = await fetchAllCursorPages<CommerceSummary>(async (cursor) => {
    const query = new URLSearchParams({
      limit: '100',
      'filter[active]': 'true',
    });
    if (cursor) {
      query.set('cursor', cursor);
    }
    return apiFetch<CursorListResponse<CommerceSummary>>(`/commerces?${query}`);
  });
  if (!search?.trim()) return commerces;
  const term = search.trim().toLowerCase();
  return commerces.filter(
    (commerce) =>
      commerce.legalName.toLowerCase().includes(term) ||
      (commerce.tradeName?.toLowerCase().includes(term) ?? false),
  );
}

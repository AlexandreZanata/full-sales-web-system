import { apiFetch } from '@/lib/api/client';
import type { CommerceSummary, PaginatedResponse } from '@/lib/api/types';

export async function fetchCommerces(search?: string): Promise<CommerceSummary[]> {
  const query = new URLSearchParams({ page: '1', pageSize: '50', active: 'true' });
  const response = await apiFetch<PaginatedResponse<CommerceSummary>>(`/commerces?${query}`);
  if (!search?.trim()) return response.items;
  const term = search.trim().toLowerCase();
  return response.items.filter(
    (commerce) =>
      commerce.legalName.toLowerCase().includes(term) ||
      (commerce.tradeName?.toLowerCase().includes(term) ?? false),
  );
}

import { apiFetch } from '@/lib/api/client';
import type { CommerceSummary, PaginatedResponse } from '@/lib/api/types';

export async function fetchCommercesForPicker(): Promise<CommerceSummary[]> {
  const query = new URLSearchParams({ page: '1', pageSize: '50', active: 'true' });
  const data = await apiFetch<PaginatedResponse<CommerceSummary>>(`/commerces?${query}`);
  return data.items;
}

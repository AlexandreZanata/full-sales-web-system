import { apiDelete, apiFetch, apiPatch, apiPost, apiPut } from '@/lib/api/client';
import {
  type CursorListParams,
  type CursorListResponse,
  fetchAllCursorPages,
} from '@/lib/cursorPagination';
import type { ActiveFilter } from '@/lib/commerces/constants';
import type {
  CategoryDetail,
  CategorySummary,
  CreateCategoryRequest,
  UpdateCategoryRequest,
} from '@/lib/api/types';

export type CategoriesListParams = CursorListParams & {
  active?: ActiveFilter;
};

function buildCategoriesQuery(params: CategoriesListParams): string {
  const query = new URLSearchParams();
  query.set('limit', String(params.limit ?? 100));
  if (params.cursor) {
    query.set('cursor', params.cursor);
  }
  if (params.active === 'true' || params.active === 'false') {
    query.set('filter[active]', params.active);
  }
  return query.toString();
}

export async function fetchCategories(
  params: CategoriesListParams,
): Promise<CursorListResponse<CategorySummary>> {
  return apiFetch<CursorListResponse<CategorySummary>>(
    `/categories?${buildCategoriesQuery(params)}`,
  );
}

export async function fetchCategoriesForPicker(): Promise<CategorySummary[]> {
  return fetchAllCursorPages(async (cursor) =>
    fetchCategories({ limit: 100, cursor, active: 'true' }),
  );
}

export async function fetchCategory(id: string): Promise<CategoryDetail> {
  return apiFetch<CategoryDetail>(`/categories/${id}`);
}

export async function createCategory(body: CreateCategoryRequest): Promise<CategoryDetail> {
  return apiPost<CategoryDetail>('/categories', body);
}

export async function updateCategory(
  id: string,
  body: UpdateCategoryRequest,
): Promise<CategoryDetail> {
  return apiPatch<CategoryDetail>(`/categories/${id}`, {
    body: JSON.stringify(body),
  });
}

export async function deactivateCategory(id: string): Promise<void> {
  await apiDelete(`/categories/${id}`);
}

export async function reorderCategories(orderedIds: string[]): Promise<void> {
  await apiPost('/categories/reorder', { orderedIds });
}

export async function uploadCategoryImage(id: string, fileId: string): Promise<CategoryDetail> {
  return apiPut<CategoryDetail>(`/categories/${id}/image`, { fileId });
}

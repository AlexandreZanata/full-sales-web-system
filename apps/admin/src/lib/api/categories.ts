import { apiDelete, apiFetch, apiPatch, apiPost, apiPut } from '@/lib/api/client';
import type { ActiveFilter } from '@/lib/commerces/constants';
import type {
  CategoryDetail,
  CategorySummary,
  CreateCategoryRequest,
  PaginatedResponse,
  UpdateCategoryRequest,
} from '@/lib/api/types';

export type CategoriesListParams = {
  page: number;
  pageSize: number;
  active?: ActiveFilter;
};

function buildActiveQuery(active?: ActiveFilter): string {
  if (active === 'true' || active === 'false') {
    return active;
  }
  return '';
}

export async function fetchCategories(
  params: CategoriesListParams,
): Promise<PaginatedResponse<CategorySummary>> {
  const query = new URLSearchParams({
    page: String(params.page),
    pageSize: String(params.pageSize),
  });
  const active = buildActiveQuery(params.active);
  if (active) {
    query.set('active', active);
  }
  return apiFetch<PaginatedResponse<CategorySummary>>(`/categories?${query}`);
}

export async function fetchCategoriesForPicker(): Promise<CategorySummary[]> {
  const data = await fetchCategories({ page: 1, pageSize: 100, active: 'true' });
  return data.items;
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

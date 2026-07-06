import { apiDelete, apiFetch, apiPatch, apiPost } from '@/lib/api/client';
import {
  type CursorListParams,
  type CursorListResponse,
  fetchAllCursorPages,
} from '@/lib/cursorPagination';
import type { ActiveFilter } from '@/lib/commerces/constants';
import type {
  AttachProductImageRequest,
  CreateProductRequest,
  Product,
  ProductImage,
  ProductSummary,
  UpdateProductRequest,
} from '@/lib/api/types';

export type ProductsListParams = CursorListParams & {
  active?: ActiveFilter;
};

function buildProductsQuery(params: ProductsListParams): string {
  const query = new URLSearchParams();
  query.set('limit', String(params.limit ?? 20));
  if (params.cursor) {
    query.set('cursor', params.cursor);
  }
  if (params.active === 'true' || params.active === 'false') {
    query.set('filter[active]', params.active);
  }
  return query.toString();
}

export async function fetchProducts(
  params: ProductsListParams,
): Promise<CursorListResponse<ProductSummary>> {
  return apiFetch<CursorListResponse<ProductSummary>>(`/products?${buildProductsQuery(params)}`);
}

export async function fetchProductsForPicker(): Promise<ProductSummary[]> {
  return fetchAllCursorPages(async (cursor) =>
    fetchProducts({ limit: 100, cursor, active: 'true' }),
  );
}

export async function fetchProduct(id: string): Promise<Product> {
  return apiFetch<Product>(`/products/${id}`);
}

export async function createProduct(body: CreateProductRequest): Promise<Product> {
  return apiPost<Product>('/products', body);
}

export async function updateProduct(id: string, body: UpdateProductRequest): Promise<Product> {
  return apiPatch<Product>(`/products/${id}`, {
    body: JSON.stringify(body),
  });
}

export async function attachProductImage(
  productId: string,
  body: AttachProductImageRequest,
): Promise<ProductImage> {
  return apiPost<ProductImage>(`/products/${productId}/images`, body);
}

export async function deleteProductImage(productId: string, imageId: string): Promise<void> {
  await apiDelete(`/products/${productId}/images/${imageId}`);
}

export async function fetchProductImages(
  productId: string,
  params: CursorListParams = {},
): Promise<CursorListResponse<ProductImage>> {
  const query = new URLSearchParams();
  query.set('limit', String(params.limit ?? 50));
  if (params.cursor) {
    query.set('cursor', params.cursor);
  }
  return apiFetch<CursorListResponse<ProductImage>>(`/products/${productId}/images?${query}`);
}

import { apiDelete, apiFetch, apiPatch, apiPost } from '@/lib/api/client';
import type {
  AttachProductImageRequest,
  CreateProductRequest,
  PaginatedResponse,
  Product,
  ProductImage,
  ProductSummary,
  UpdateProductRequest,
} from '@/lib/api/types';

export type ProductsListParams = {
  page: number;
  pageSize: number;
};

export async function fetchProducts(
  params: ProductsListParams,
): Promise<PaginatedResponse<ProductSummary>> {
  const query = new URLSearchParams({
    page: String(params.page),
    pageSize: String(params.pageSize),
  });
  return apiFetch<PaginatedResponse<ProductSummary>>(`/products?${query}`);
}

export async function fetchProductsForPicker(): Promise<ProductSummary[]> {
  const data = await fetchProducts({ page: 1, pageSize: 50 });
  return data.items;
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

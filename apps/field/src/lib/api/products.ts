import { apiFetch } from '@/lib/api/client';
import type { PaginatedResponse, ProductSummary, StockBalance } from '@/lib/api/types';

export async function fetchProducts(): Promise<ProductSummary[]> {
  const query = new URLSearchParams({ page: '1', pageSize: '50' });
  const response = await apiFetch<PaginatedResponse<ProductSummary>>(`/products?${query}`);
  return response.items.filter((product) => product.active);
}

export async function fetchStockBalance(productId: string): Promise<StockBalance> {
  return apiFetch<StockBalance>(`/inventory/products/${productId}/balance`);
}

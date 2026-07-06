import { apiFetch } from '@/lib/api/client';
import { fetchAllCursorPages, type CursorListResponse } from '@/lib/cursorPagination';
import type { ProductSummary, StockBalance } from '@/lib/api/types';

export async function fetchProducts(): Promise<ProductSummary[]> {
  const products = await fetchAllCursorPages<ProductSummary>(async (cursor) => {
    const query = new URLSearchParams({
      limit: '100',
      'filter[active]': 'true',
    });
    if (cursor) {
      query.set('cursor', cursor);
    }
    return apiFetch<CursorListResponse<ProductSummary>>(`/products?${query}`);
  });
  return products.filter((product) => product.active);
}

export async function fetchStockBalance(productId: string): Promise<StockBalance> {
  return apiFetch<StockBalance>(`/inventory/products/${productId}/balance`);
}

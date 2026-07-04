/**
 * Build a lookup map from product picker data for line-item labels.
 */
import type { ProductSummary } from '@/lib/api/types';

export function buildProductNameMap(products: ProductSummary[]): Map<string, string> {
  const map = new Map<string, string>();
  for (const product of products) {
    map.set(product.id, `${product.sku} — ${product.name}`);
  }
  return map;
}

export function productDisplayName(map: Map<string, string>, productId: string): string {
  return map.get(productId) ?? `${productId.slice(0, 8)}…`;
}

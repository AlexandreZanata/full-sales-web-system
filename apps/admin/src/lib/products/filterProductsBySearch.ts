import type { ProductSummary } from '@/lib/api/types';

export function formatProductOption(product: Pick<ProductSummary, 'sku' | 'name'>): string {
  return `${product.sku} — ${product.name}`;
}

/** Lowercase + strip diacritics so "guarana" matches "Guaraná". */
export function normalizeSearchText(value: string): string {
  return value
    .trim()
    .toLowerCase()
    .normalize('NFD')
    .replace(/\p{M}/gu, '');
}

/** Match product name or SKU (case/accent-insensitive substring). Empty query returns all. */
export function filterProductsBySearch(
  products: ProductSummary[],
  search: string,
): ProductSummary[] {
  const normalized = normalizeSearchText(search);
  if (!normalized) {
    return products;
  }
  return products.filter((product) => {
    const name = normalizeSearchText(product.name);
    const sku = normalizeSearchText(product.sku);
    return name.includes(normalized) || sku.includes(normalized);
  });
}

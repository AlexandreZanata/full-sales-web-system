import type { PortalCategory, PortalProduct } from '@/lib/api/types';

export type CatalogSearchParams = {
  category?: string;
};

export function parseCatalogSearch(search: Record<string, unknown>): CatalogSearchParams {
  return {
    category: typeof search.category === 'string' ? search.category : undefined,
  };
}

export function resolveDefaultCategorySlug(categories: PortalCategory[]): string | undefined {
  return categories
    .filter((category) => category.active)
    .sort((left, right) => left.sortOrder - right.sortOrder)[0]?.slug;
}

export function resolveActiveCategorySlug(
  categoryParam: string | undefined,
  categories: PortalCategory[],
): string | undefined {
  if (!categoryParam) {
    return undefined;
  }
  return categories.find((category) => category.active && category.slug === categoryParam)?.slug;
}

export function isKnownCategorySlug(categoryParam: string, categories: PortalCategory[]): boolean {
  return categories.some((category) => category.active && category.slug === categoryParam);
}

export function shouldRedirectToDefaultCategory(
  categoryParam: string | undefined,
  categories: PortalCategory[],
): boolean {
  return !categoryParam && resolveDefaultCategorySlug(categories) !== undefined;
}

export function filterProductsBySearch(products: PortalProduct[], term: string): PortalProduct[] {
  const normalized = term.trim().toLowerCase();
  if (!normalized) {
    return products;
  }
  return products.filter(
    (product) =>
      product.name.toLowerCase().includes(normalized) ||
      product.sku.toLowerCase().includes(normalized),
  );
}

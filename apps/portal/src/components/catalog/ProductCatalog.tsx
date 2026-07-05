import { useMemo, useState, type ReactNode } from 'react';

import { CatalogEmptyState } from '@/components/catalog/CatalogEmptyState';
import { CatalogSkeleton } from '@/components/catalog/CatalogSkeleton';
import { CatalogToolbar } from '@/components/catalog/CatalogToolbar';
import { CategoryBar, type CategoryBarVariant } from '@/components/catalog/CategoryBar';
import { ProductCardGrid } from '@/components/catalog/ProductCardGrid';
import { ProductCardList } from '@/components/catalog/ProductCardList';
import type { PortalCategory, PortalProduct } from '@/lib/api/types';
import { DEFAULT_CATALOG_VIEW_MODE, type CatalogViewMode } from '@/lib/catalog/viewMode';

type ProductCatalogProps = {
  categories?: PortalCategory[];
  products: PortalProduct[];
  activeCategorySlug?: string;
  categoryTitle?: string;
  onCategorySelect?: (slug: string) => void;
  onCategoryClear?: () => void;
  onAddToCart: (product: PortalProduct) => void;
  onOpenDetail?: (product: PortalProduct) => void;
  isLoading?: boolean;
  searchSlot?: ReactNode;
  emptyTitle: string;
  emptyDescription?: string;
  emptyAction?: ReactNode;
  addToCartLabel: string;
  skuLabel: string;
  allCategoriesLabel?: string;
  listViewLabel: string;
  gridViewLabel: string;
  categoryBarVariant?: CategoryBarVariant;
  initialViewMode?: CatalogViewMode;
};

export function ProductCatalog({
  categories = [],
  products,
  activeCategorySlug,
  categoryTitle,
  onCategorySelect,
  onCategoryClear,
  onAddToCart,
  onOpenDetail,
  isLoading = false,
  searchSlot,
  emptyTitle,
  emptyDescription,
  emptyAction,
  addToCartLabel,
  skuLabel,
  allCategoriesLabel,
  listViewLabel,
  gridViewLabel,
  categoryBarVariant = 'menu',
  initialViewMode = DEFAULT_CATALOG_VIEW_MODE,
}: ProductCatalogProps) {
  const [viewMode, setViewMode] = useState<CatalogViewMode>(initialViewMode);

  const toolbarTitle = useMemo(() => {
    if (categoryTitle) {
      return categoryTitle;
    }
    if (activeCategorySlug) {
      return categories.find((category) => category.slug === activeCategorySlug)?.name;
    }
    return undefined;
  }, [activeCategorySlug, categories, categoryTitle]);

  if (isLoading) {
    return <CatalogSkeleton viewMode={viewMode} />;
  }

  return (
    <div className="space-y-4">
      {categories.length > 0 ? (
        <CategoryBar
          categories={categories}
          activeSlug={activeCategorySlug}
          variant={categoryBarVariant}
          allLabel={allCategoriesLabel}
          onSelectAll={onCategoryClear}
          onSelect={(slug) => {
            onCategorySelect?.(slug);
          }}
        />
      ) : null}

      <CatalogToolbar
        title={toolbarTitle}
        viewMode={viewMode}
        onViewModeChange={setViewMode}
        listViewLabel={listViewLabel}
        gridViewLabel={gridViewLabel}
        searchSlot={searchSlot}
      />

      {products.length === 0 ? (
        <CatalogEmptyState title={emptyTitle} description={emptyDescription} action={emptyAction} />
      ) : viewMode === 'grid' ? (
        <div
          className="grid grid-cols-2 gap-3 md:grid-cols-3 lg:grid-cols-4 lg:gap-4"
          data-testid="catalog-product-grid"
        >
          {products.map((product) => (
            <ProductCardGrid
              key={product.id}
              product={product}
              onAddToCart={onAddToCart}
              onOpenDetail={onOpenDetail}
              addToCartLabel={addToCartLabel}
              skuLabel={skuLabel}
            />
          ))}
        </div>
      ) : (
        <div className="space-y-3" data-testid="catalog-product-list">
          {products.map((product) => (
            <ProductCardList
              key={product.id}
              product={product}
              onAddToCart={onAddToCart}
              onOpenDetail={onOpenDetail}
              addToCartLabel={addToCartLabel}
              skuLabel={skuLabel}
            />
          ))}
        </div>
      )}
    </div>
  );
}

import { useQuery } from '@tanstack/react-query';
import { useNavigate } from '@tanstack/react-router';
import { useMemo, useState } from 'react';

import { useCart } from '@/cart/CartProvider';
import { CartFab } from '@/components/CartFab';
import { CatalogEmptyState } from '@/components/catalog/CatalogEmptyState';
import { CatalogSkeleton } from '@/components/catalog/CatalogSkeleton';
import { CatalogSearchField } from '@/components/catalog/CatalogSearchField';
import { ProductCatalog } from '@/components/catalog/ProductCatalog';
import { Button } from '@/components/ui/Button';
import { fetchPortalCategoryBySlug } from '@/lib/api/portal';
import {
  filterProductsBySearch,
  resolveActiveCategorySlug,
  resolveDefaultCategorySlug,
} from '@/lib/catalog/catalogSearch';
import { useCatalogCategories } from '@/lib/catalog/useCatalogCategories';
import { useDebouncedValue } from '@/lib/catalog/useDebouncedValue';
import { useI18n } from '@/lib/i18n/context';

type CatalogPageContentProps = {
  categoryParam?: string;
};

export function CatalogPageContent({ categoryParam }: CatalogPageContentProps) {
  const { t } = useI18n();
  const navigate = useNavigate();
  const { addProduct } = useCart();
  const categoriesQuery = useCatalogCategories();
  const categories = categoriesQuery.data ?? [];
  const activeSlug = resolveActiveCategorySlug(categoryParam, categories);
  const defaultSlug = resolveDefaultCategorySlug(categories);
  const isUnknownSlug = Boolean(categoryParam && categories.length > 0 && !activeSlug);
  const [searchInput, setSearchInput] = useState('');
  const debouncedSearch = useDebouncedValue(searchInput, 300);

  const categoryQuery = useQuery({
    queryKey: ['portal', 'category', activeSlug],
    queryFn: () => {
      if (!activeSlug) {
        throw new Error('Category slug is required');
      }
      return fetchPortalCategoryBySlug(activeSlug);
    },
    enabled: Boolean(activeSlug),
  });

  const filteredProducts = useMemo(
    () => filterProductsBySearch(categoryQuery.data?.products ?? [], debouncedSearch),
    [categoryQuery.data?.products, debouncedSearch],
  );

  if (categoriesQuery.isLoading || (activeSlug && categoryQuery.isLoading)) {
    return <CatalogSkeleton />;
  }

  if (categoriesQuery.isError || categoryQuery.isError) {
    return (
      <CatalogEmptyState
        title={t('common.error.loadFailed')}
        action={
          <Button
            onClick={() => {
              void categoriesQuery.refetch();
              void categoryQuery.refetch();
            }}
          >
            {t('common.tryAgain')}
          </Button>
        }
      />
    );
  }

  if (isUnknownSlug && defaultSlug) {
    return (
      <CatalogEmptyState
        title={t('catalog.emptyCategory')}
        description={t('catalog.selectCategory')}
        action={
          <Button
            onClick={() => {
              void navigate({ to: '/', search: { category: defaultSlug } });
            }}
          >
            {t('catalog.selectCategory')}
          </Button>
        }
      />
    );
  }

  return (
    <div className="space-y-4">
      <h1 className="text-2xl font-semibold text-foreground">{t('catalog.title')}</h1>
      <ProductCatalog
        categories={categories}
        products={filteredProducts}
        activeCategorySlug={activeSlug}
        categoryTitle={categoryQuery.data?.name}
        onCategorySelect={(slug) => {
          void navigate({ to: '/', search: { category: slug } });
        }}
        onAddToCart={addProduct}
        onOpenDetail={(product) => {
          void navigate({
            to: '/products/$id',
            params: { id: product.id },
            search: activeSlug ? { category: activeSlug } : undefined,
          });
        }}
        searchSlot={
          <CatalogSearchField
            label={t('common.search')}
            placeholder={t('catalog.searchPlaceholder')}
            value={searchInput}
            onChange={(event) => {
              setSearchInput(event.target.value);
            }}
          />
        }
        emptyTitle={t('common.empty.products')}
        emptyDescription={t('catalog.emptyDescription')}
        addToCartLabel={t('common.addToCart')}
        skuLabel={t('catalog.sku')}
        listViewLabel={t('catalog.viewList')}
        gridViewLabel={t('catalog.viewGrid')}
        categoriesAriaLabel={t('catalog.categories')}
      />
      <CartFab />
    </div>
  );
}

import { useInfiniteQuery } from '@tanstack/react-query';
import { Link, useNavigate } from '@tanstack/react-router';
import { useEffect, useMemo, useState } from 'react';

import { useCart } from '@/cart/CartProvider';
import { CatalogEmptyState } from '@/components/catalog/CatalogEmptyState';
import { CatalogSearchField } from '@/components/catalog/CatalogSearchField';
import { CatalogSkeleton } from '@/components/catalog/CatalogSkeleton';
import { ProductCatalog } from '@/components/catalog/ProductCatalog';
import { Button } from '@/components/ui/Button';
import { fetchPortalCategoryBySlug } from '@/lib/api/portal';
import {
  catalogHomeSearch,
  filterProductsBySearch,
  resolveActiveCategorySlug,
  resolveDefaultCategorySlug,
} from '@/lib/catalog/catalogSearch';
import { useCatalogCategories } from '@/lib/catalog/useCatalogCategories';
import { useDebouncedValue } from '@/lib/catalog/useDebouncedValue';
import { useI18n } from '@/lib/i18n/context';

type CatalogPageContentProps = {
  categoryParam?: string;
  initialSearch?: string;
};

export function CatalogPageContent({ categoryParam, initialSearch }: CatalogPageContentProps) {
  const { t } = useI18n();
  const navigate = useNavigate();
  const { addProduct } = useCart();
  const categoriesQuery = useCatalogCategories();
  const categories = categoriesQuery.data ?? [];
  const activeSlug = resolveActiveCategorySlug(categoryParam, categories);
  const defaultSlug = resolveDefaultCategorySlug(categories);
  const isUnknownSlug = Boolean(categoryParam && categories.length > 0 && !activeSlug);
  const [searchInput, setSearchInput] = useState(initialSearch ?? '');
  const debouncedSearch = useDebouncedValue(searchInput, 300);

  useEffect(() => {
    setSearchInput(initialSearch ?? '');
  }, [initialSearch]);

  useEffect(() => {
    if (!activeSlug) {
      return;
    }
    const trimmed = debouncedSearch.trim();
    const nextQuery = trimmed || undefined;
    const currentQuery = initialSearch?.trim() || undefined;
    if (nextQuery === currentQuery) {
      return;
    }
    void navigate({
      to: '/',
      search: { category: activeSlug, q: nextQuery },
      replace: true,
    });
  }, [activeSlug, debouncedSearch, initialSearch, navigate]);

  const categoryQuery = useInfiniteQuery({
    queryKey: ['portal', 'category', activeSlug],
    queryFn: ({ pageParam }) => {
      if (!activeSlug) {
        throw new Error('Category slug is required');
      }
      return fetchPortalCategoryBySlug(activeSlug, {
        limit: 50,
        cursor: pageParam,
      });
    },
    initialPageParam: undefined as string | undefined,
    getNextPageParam: (lastPage) =>
      lastPage.pagination.has_more ? (lastPage.pagination.next_cursor ?? undefined) : undefined,
    enabled: Boolean(activeSlug),
  });

  const categoryProducts = useMemo(
    () => categoryQuery.data?.pages.flatMap((page) => page.products) ?? [],
    [categoryQuery.data?.pages],
  );

  const categoryTitle = categoryQuery.data?.pages[0]?.name;

  const filteredProducts = useMemo(
    () => filterProductsBySearch(categoryProducts, debouncedSearch),
    [categoryProducts, debouncedSearch],
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

  const showLoadMore = categoryQuery.hasNextPage && !debouncedSearch.trim();

  return (
    <div className="space-y-4" data-testid="catalog-menu">
      <div className="flex flex-wrap items-center justify-between gap-3">
        <h1 className="text-2xl font-semibold text-foreground">{t('catalog.title')}</h1>
        <Link to="/" search={catalogHomeSearch} className="catalog-back-home-link">
          {t('catalog.backToHome')}
        </Link>
      </div>
      <ProductCatalog
        categories={categories}
        products={filteredProducts}
        activeCategorySlug={activeSlug}
        categoryTitle={categoryTitle}
        onCategorySelect={(slug) => {
          void navigate({
            to: '/',
            search: { category: slug, q: debouncedSearch.trim() || undefined },
          });
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
        categoryBarVariant="menu"
      />
      {showLoadMore ? (
        <div className="flex justify-center">
          <Button
            variant="secondary"
            onClick={() => {
              void categoryQuery.fetchNextPage();
            }}
            disabled={categoryQuery.isFetchingNextPage}
          >
            {t('common.loadMore')}
          </Button>
        </div>
      ) : null}
    </div>
  );
}

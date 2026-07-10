import { useQuery } from '@tanstack/react-query';
import { Link, createFileRoute } from '@tanstack/react-router';
import { ChevronRight } from 'lucide-react';

import { ProductDetailActions } from '@/components/catalog/ProductDetailActions';
import { ProductDetailInfo } from '@/components/catalog/ProductDetailInfo';
import { ProductDetailSkeleton } from '@/components/catalog/ProductDetailSkeleton';
import { ProductMediaPanel } from '@/components/catalog/ProductMediaPanel';
import { EmptyState } from '@/components/ui/EmptyState';
import { fetchPortalProductById } from '@/lib/api/portal';
import { useI18n } from '@/lib/i18n/context';
import { useSiteSettings } from '@/lib/settings/useSiteSettings';
import { cn } from '@/lib/utils';

type ProductDetailSearch = {
  category?: string;
};

function parseProductDetailSearch(search: Record<string, unknown>): ProductDetailSearch {
  return {
    category: typeof search.category === 'string' ? search.category : undefined,
  };
}

export const Route = createFileRoute('/_authenticated/products/$id')({
  validateSearch: parseProductDetailSearch,
  component: ProductDetailPage,
});

function ProductDetailPage() {
  const { id } = Route.useParams();
  const { category: categorySlug } = Route.useSearch();
  const { t } = useI18n();
  const settingsQuery = useSiteSettings(true);

  const productQuery = useQuery({
    queryKey: ['portal', 'product', id],
    queryFn: () => fetchPortalProductById(id),
  });

  const product = productQuery.data;

  if (productQuery.isLoading) {
    return <ProductDetailSkeleton />;
  }

  if (!product) {
    return <EmptyState title={t('common.empty.products')} />;
  }

  const catalogSearch = product.categorySlug ?? categorySlug;
  const productUrl =
    typeof window !== 'undefined'
      ? `${window.location.origin}/products/${product.id}${catalogSearch ? `?category=${encodeURIComponent(catalogSearch)}` : ''}`
      : `/products/${product.id}`;

  return (
    <div className="space-y-6 pb-24 md:pb-0">
      <nav aria-label="Breadcrumb" className="catalog-breadcrumb">
        <ol className="catalog-breadcrumb__list">
          <li>
            <Link
              to="/"
              search={catalogSearch ? { category: catalogSearch } : undefined}
              className="catalog-breadcrumb__link"
            >
              {t('nav.catalog')}
            </Link>
          </li>
          {product.categoryName ? (
            <>
              <li aria-hidden className="flex items-center">
                <ChevronRight className="catalog-breadcrumb__sep" />
              </li>
              <li>
                <Link
                  to="/"
                  search={catalogSearch ? { category: catalogSearch } : undefined}
                  className="catalog-breadcrumb__link"
                >
                  {product.categoryName}
                </Link>
              </li>
            </>
          ) : null}
          <li aria-hidden className="flex items-center">
            <ChevronRight className="catalog-breadcrumb__sep" />
          </li>
          <li className="catalog-breadcrumb__current">{product.name}</li>
        </ol>
      </nav>

      <div className="grid gap-8 lg:grid-cols-2">
        <ProductMediaPanel product={product} />
        <ProductDetailInfo product={product} />
      </div>

      <div
        className={cn(
          'fixed inset-x-0 bottom-16 z-10 border-t border-hairline bg-surface/95 p-4 backdrop-blur md:static md:border-0 md:bg-transparent md:p-0 md:backdrop-blur-none',
        )}
      >
        <ProductDetailActions
          product={product}
          salesContactPhone={settingsQuery.data?.salesContactPhone}
          productUrl={productUrl}
        />
      </div>
    </div>
  );
}

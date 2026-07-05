import { useQuery } from '@tanstack/react-query';
import { Link, createFileRoute } from '@tanstack/react-router';

import { useCart } from '@/cart/CartProvider';
import { ProductImage } from '@/components/catalog/ProductImage';
import { Button } from '@/components/ui/Button';
import { EmptyState } from '@/components/ui/EmptyState';
import { LoadingSpinner } from '@/components/ui/LoadingSpinner';
import { fetchPortalProductById } from '@/lib/api/portal';
import { useI18n } from '@/lib/i18n/context';
import { formatMoney } from '@/lib/products/formatPrice';
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
  const { addProduct } = useCart();

  const productQuery = useQuery({
    queryKey: ['portal', 'product', id, categorySlug ?? 'auto'],
    queryFn: () => fetchPortalProductById(id, categorySlug),
  });

  const product = productQuery.data;

  if (productQuery.isLoading) {
    return <LoadingSpinner className="py-16" />;
  }

  if (!product) {
    return <EmptyState title={t('common.empty.products')} />;
  }

  const catalogSearch = product.categorySlug ?? categorySlug;

  return (
    <div className="mx-auto max-w-2xl space-y-4 pb-24 md:pb-0">
      <nav aria-label="Breadcrumb" className="text-sm text-muted-foreground">
        <ol className="flex flex-wrap items-center gap-1">
          <li>
            <Link
              to="/"
              search={catalogSearch ? { category: catalogSearch } : undefined}
              className="hover:text-foreground"
            >
              {t('nav.catalog')}
            </Link>
          </li>
          {product.categoryName ? (
            <>
              <li aria-hidden>→</li>
              <li>
                <Link
                  to="/"
                  search={catalogSearch ? { category: catalogSearch } : undefined}
                  className="hover:text-foreground"
                >
                  {product.categoryName}
                </Link>
              </li>
            </>
          ) : null}
          <li aria-hidden>→</li>
          <li className="text-foreground">{product.name}</li>
        </ol>
      </nav>

      <article className="overflow-hidden rounded-lg border border-hairline bg-surface">
        <ProductImage product={product} className="aspect-[4/3] w-full md:aspect-square" />
        <div className="space-y-4 p-4 md:p-6">
          <div className="space-y-2">
            <span className="inline-flex rounded-full border border-hairline bg-surface-muted px-2.5 py-0.5 text-xs font-medium text-muted-foreground">
              {t('catalog.sku')}: {product.sku}
            </span>
            <h1 className="text-2xl font-semibold text-foreground">{product.name}</h1>
            {product.categoryName ? (
              <p className="text-sm text-muted-foreground">{product.categoryName}</p>
            ) : null}
          </div>
          <p className="catalog-price catalog-price--prominent text-xl">
            {formatMoney(product.priceAmount, product.priceCurrency)}
          </p>
          <p className="text-sm text-muted-foreground">
            {t('catalog.unitPrice')}: {formatMoney(product.priceAmount, product.priceCurrency)}
          </p>
        </div>
      </article>

      <div
        className={cn(
          'fixed inset-x-0 bottom-16 z-10 border-t border-hairline bg-surface/95 p-4 backdrop-blur md:static md:border-0 md:bg-transparent md:p-0 md:backdrop-blur-none',
        )}
      >
        <Button
          className="w-full md:max-w-sm"
          onClick={() => {
            addProduct(product);
          }}
        >
          {t('common.addToCart')}
        </Button>
      </div>
    </div>
  );
}

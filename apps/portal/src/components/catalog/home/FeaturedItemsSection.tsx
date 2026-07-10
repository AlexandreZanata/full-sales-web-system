import { useNavigate } from '@tanstack/react-router';

import { useCart } from '@/cart/CartProvider';
import { ProductCardGrid } from '@/components/catalog/ProductCardGrid';
import { useFeaturedProducts } from '@/lib/catalog/useFeaturedProducts';
import { useI18n } from '@/lib/i18n/context';

function FeaturedItemsSkeleton() {
  return (
    <section data-testid="featured-items" aria-busy="true" className="mb-8">
      <div className="mb-6 h-8 w-48 animate-pulse rounded-lg bg-surface-muted" />
      <div className="grid grid-cols-2 gap-3 sm:grid-cols-3 md:grid-cols-4 lg:gap-6">
        {Array.from({ length: 4 }, (_, index) => (
          <div
            key={`featured-skeleton-${String(index)}`}
            className="flex flex-col overflow-hidden rounded-2xl border border-hairline"
          >
            <div className="aspect-[4/3] animate-pulse bg-surface-muted" />
            <div className="space-y-2 p-3">
              <div className="h-4 w-3/4 animate-pulse rounded bg-surface-muted" />
              <div className="h-3 w-full animate-pulse rounded bg-surface-muted" />
              <div className="h-8 w-full animate-pulse rounded-3xl bg-surface-muted" />
            </div>
          </div>
        ))}
      </div>
    </section>
  );
}

export function FeaturedItemsSection() {
  const { t } = useI18n();
  const navigate = useNavigate();
  const { addProduct } = useCart();
  const featuredQuery = useFeaturedProducts(12);
  const products = featuredQuery.data ?? [];

  if (featuredQuery.isLoading) {
    return <FeaturedItemsSkeleton />;
  }

  if (products.length === 0) {
    return null;
  }

  return (
    <section data-testid="featured-items" className="mb-8">
      <h2 className="mb-6 text-2xl font-semibold capitalize text-foreground">
        {t('catalog.featuredItems')}
      </h2>
      <div className="grid grid-cols-2 gap-3 sm:grid-cols-3 md:grid-cols-4 lg:gap-6">
        {products.map((product) => (
          <ProductCardGrid
            key={product.id}
            product={product}
            onAddToCart={addProduct}
            onOpenDetail={(item) => {
              void navigate({
                to: '/products/$id',
                params: { id: item.id },
                search: item.categorySlug ? { category: item.categorySlug } : undefined,
              });
            }}
            addToCartLabel={t('common.addToCart')}
            skuLabel={t('catalog.sku')}
          />
        ))}
      </div>
    </section>
  );
}

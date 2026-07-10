import { useNavigate } from '@tanstack/react-router';

import { useCart } from '@/cart/CartProvider';
import { ProductCardList } from '@/components/catalog/ProductCardList';
import { usePopularProducts } from '@/lib/catalog/usePopularProducts';
import { useI18n } from '@/lib/i18n/context';

function PopularItemsSkeleton() {
  return (
    <section data-testid="popular-items" aria-busy="true" className="mb-8">
      <div className="mb-6 h-8 w-56 animate-pulse rounded-lg bg-surface-muted" />
      <div className="grid grid-cols-1 gap-3 sm:grid-cols-2 md:gap-6 lg:grid-cols-3">
        {Array.from({ length: 3 }, (_, index) => (
          <div
            key={`popular-skeleton-${String(index)}`}
            className="h-36 animate-pulse rounded-xl border border-hairline bg-surface-muted"
          />
        ))}
      </div>
    </section>
  );
}

export function PopularItemsSection() {
  const { t } = useI18n();
  const navigate = useNavigate();
  const { addProduct } = useCart();
  const popularQuery = usePopularProducts(12);
  const products = popularQuery.data ?? [];

  if (popularQuery.isLoading) {
    return <PopularItemsSkeleton />;
  }

  if (products.length === 0) {
    return null;
  }

  return (
    <section data-testid="popular-items" className="mb-8">
      <h2 className="mb-6 text-2xl font-semibold capitalize text-foreground">
        {t('catalog.popularItems')}
      </h2>
      <div className="grid grid-cols-1 gap-3 sm:grid-cols-2 md:gap-6 lg:grid-cols-3">
        {products.map((product) => (
          <ProductCardList
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

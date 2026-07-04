import { useQuery } from '@tanstack/react-query';
import { Link, createFileRoute } from '@tanstack/react-router';
import { useMemo, useState } from 'react';

import { useCart } from '@/cart/CartProvider';
import { CartFab } from '@/components/CartFab';
import { Button } from '@/components/ui/Button';
import { EmptyState } from '@/components/ui/EmptyState';
import { Input } from '@/components/ui/Input';
import { LoadingSpinner } from '@/components/ui/LoadingSpinner';
import { fetchPortalProducts } from '@/lib/api/portal';
import { useI18n } from '@/lib/i18n/context';
import { formatMoney } from '@/lib/products/formatPrice';

export const Route = createFileRoute('/_authenticated/')({
  component: CatalogPage,
});

function CatalogPage() {
  const { t } = useI18n();
  const { addProduct } = useCart();
  const [search, setSearch] = useState('');

  const productsQuery = useQuery({
    queryKey: ['portal', 'products'],
    queryFn: () => fetchPortalProducts({ pageSize: 50 }),
  });

  const filtered = useMemo(() => {
    const items = productsQuery.data?.items ?? [];
    const term = search.trim().toLowerCase();
    if (!term) {
      return items;
    }
    return items.filter(
      (product) =>
        product.name.toLowerCase().includes(term) || product.sku.toLowerCase().includes(term),
    );
  }, [productsQuery.data?.items, search]);

  if (productsQuery.isLoading) {
    return <LoadingSpinner className="py-16" />;
  }

  if (productsQuery.isError) {
    return (
      <EmptyState
        title={t('common.error.loadFailed')}
        action={
          <Button onClick={() => void productsQuery.refetch()}>{t('common.tryAgain')}</Button>
        }
      />
    );
  }

  return (
    <div className="space-y-4">
      <div className="flex flex-col gap-3 sm:flex-row sm:items-end sm:justify-between">
        <h1 className="text-2xl font-semibold text-foreground">{t('catalog.title')}</h1>
        <Input
          label={t('common.search')}
          placeholder={t('catalog.searchPlaceholder')}
          value={search}
          onChange={(event) => {
            setSearch(event.target.value);
          }}
          className="sm:max-w-xs"
        />
      </div>

      {filtered.length === 0 ? (
        <EmptyState title={t('common.empty.products')} />
      ) : (
        <div className="grid grid-cols-2 gap-3 md:grid-cols-3 lg:grid-cols-4 lg:gap-4">
          {filtered.map((product) => (
            <article
              key={product.id}
              className="flex flex-col overflow-hidden rounded-lg border border-hairline bg-surface"
            >
              <Link to="/products/$id" params={{ id: product.id }} className="block">
                <div className="aspect-square bg-surface-muted">
                  {product.primaryImageUrl ? (
                    <img
                      src={product.primaryImageUrl}
                      alt=""
                      className="size-full object-cover"
                      loading="lazy"
                    />
                  ) : (
                    <div className="flex size-full items-center justify-center text-xs text-muted-foreground">
                      {product.sku}
                    </div>
                  )}
                </div>
                <div className="space-y-1 p-3">
                  <h2 className="line-clamp-2 text-sm font-semibold text-foreground">
                    {product.name}
                  </h2>
                  <p className="text-sm font-medium text-foreground">
                    {formatMoney(product.priceAmount, product.priceCurrency)}
                  </p>
                </div>
              </Link>
              <div className="mt-auto p-3 pt-0">
                <Button
                  className="w-full"
                  variant="secondary"
                  onClick={() => {
                    addProduct(product);
                  }}
                >
                  {t('common.addToCart')}
                </Button>
              </div>
            </article>
          ))}
        </div>
      )}

      <CartFab />
    </div>
  );
}

import { useQuery } from '@tanstack/react-query';
import { Link, createFileRoute } from '@tanstack/react-router';

import { useCart } from '@/cart/CartProvider';
import { Button } from '@/components/ui/Button';
import { EmptyState } from '@/components/ui/EmptyState';
import { LoadingSpinner } from '@/components/ui/LoadingSpinner';
import { fetchPortalProducts } from '@/lib/api/portal';
import { useI18n } from '@/lib/i18n/context';
import { formatMoney } from '@/lib/products/formatPrice';

export const Route = createFileRoute('/_authenticated/products/$id')({
  component: ProductDetailPage,
});

function ProductDetailPage() {
  const { id } = Route.useParams();
  const { t } = useI18n();
  const { addProduct } = useCart();

  const productsQuery = useQuery({
    queryKey: ['portal', 'products'],
    queryFn: () => fetchPortalProducts({ pageSize: 50 }),
  });

  const product = productsQuery.data?.items.find((item) => item.id === id);

  if (productsQuery.isLoading) {
    return <LoadingSpinner className="py-16" />;
  }

  if (!product) {
    return <EmptyState title={t('common.empty.products')} />;
  }

  return (
    <div className="mx-auto max-w-lg space-y-4">
      <Link to="/" className="text-sm text-muted-foreground hover:text-foreground">
        ← {t('common.backToCatalog')}
      </Link>
      <div className="overflow-hidden rounded-lg border border-hairline bg-surface">
        <div className="aspect-square bg-surface-muted">
          {product.primaryImageUrl ? (
            <img src={product.primaryImageUrl} alt="" className="size-full object-cover" />
          ) : (
            <div className="flex size-full items-center justify-center text-sm text-muted-foreground">
              {product.sku}
            </div>
          )}
        </div>
        <div className="space-y-3 p-4">
          <h1 className="text-xl font-semibold text-foreground">{product.name}</h1>
          <p className="text-sm text-muted-foreground">
            {t('catalog.sku')}: {product.sku}
          </p>
          <p className="text-lg font-semibold text-foreground">
            {formatMoney(product.priceAmount, product.priceCurrency)}
          </p>
          <Button
            className="w-full"
            onClick={() => {
              addProduct(product);
            }}
          >
            {t('common.addToCart')}
          </Button>
        </div>
      </div>
    </div>
  );
}

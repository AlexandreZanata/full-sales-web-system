import { Button } from '@/components/ui/Button';
import type { ProductCardProps } from '@/components/catalog/productCardProps';
import { ProductImage } from '@/components/catalog/ProductImage';
import { formatMoney } from '@/lib/products/formatPrice';

function productMetaLine(product: ProductCardProps['product'], skuLabel: string): string {
  const sku = `${skuLabel}: ${product.sku}`;
  return product.categoryName ? `${sku} · ${product.categoryName}` : sku;
}

export function ProductCardList({
  product,
  onAddToCart,
  onOpenDetail,
  addToCartLabel,
  skuLabel,
}: ProductCardProps) {
  const openDetail = () => {
    onOpenDetail?.(product);
  };

  return (
    <article className="catalog-product-card-list">
      <button
        type="button"
        className="shrink-0 self-start focus-visible:outline-none"
        onClick={openDetail}
        disabled={!onOpenDetail}
        aria-label={product.name}
      >
        <ProductImage product={product} className="size-28 rounded-lg sm:size-32" />
      </button>
      <div className="flex min-w-0 flex-1 flex-col gap-2.5 sm:gap-3">
        <div className="min-w-0">
          <button
            type="button"
            className="w-full text-left focus-visible:outline-none"
            onClick={openDetail}
            disabled={!onOpenDetail}
          >
            <h3 className="line-clamp-2 text-base font-semibold leading-snug text-foreground">
              {product.name}
            </h3>
          </button>
          <p className="mt-1 text-xs text-muted-foreground">{productMetaLine(product, skuLabel)}</p>
        </div>
        <div className="mt-auto flex flex-col gap-2 sm:flex-row sm:items-center sm:justify-between sm:gap-3">
          <p className="catalog-price catalog-price--prominent">
            {formatMoney(product.priceAmount, product.priceCurrency)}
          </p>
          <Button
            className="w-full sm:w-auto sm:shrink-0"
            variant="secondary"
            onClick={() => {
              onAddToCart(product);
            }}
          >
            {addToCartLabel}
          </Button>
        </div>
      </div>
    </article>
  );
}

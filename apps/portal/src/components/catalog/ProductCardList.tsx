import { Button } from '@/components/ui/Button';
import type { ProductCardProps } from '@/components/catalog/productCardProps';
import { ProductImage } from '@/components/catalog/ProductImage';
import { formatMoney } from '@/lib/products/formatPrice';

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
        className="shrink-0 focus-visible:outline-none"
        onClick={openDetail}
        disabled={!onOpenDetail}
      >
        <ProductImage product={product} className="size-20 rounded-md sm:size-24" />
      </button>
      <div className="flex min-w-0 flex-1 flex-col justify-between gap-2">
        <div className="min-w-0 space-y-1">
          <button
            type="button"
            className="text-left focus-visible:outline-none"
            onClick={openDetail}
            disabled={!onOpenDetail}
          >
            <h3 className="truncate text-sm font-semibold text-foreground">{product.name}</h3>
          </button>
          <p className="text-xs text-muted-foreground">
            {skuLabel}: {product.sku}
          </p>
          {product.categoryName ? (
            <p className="text-xs text-muted-foreground">{product.categoryName}</p>
          ) : null}
          <p className="catalog-price">{formatMoney(product.priceAmount, product.priceCurrency)}</p>
        </div>
        <Button
          className="self-start"
          variant="secondary"
          onClick={() => {
            onAddToCart(product);
          }}
        >
          {addToCartLabel}
        </Button>
      </div>
    </article>
  );
}

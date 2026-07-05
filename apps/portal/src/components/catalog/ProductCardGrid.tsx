import { Button } from '@/components/ui/Button';
import type { ProductCardProps } from '@/components/catalog/productCardProps';
import { ProductImage } from '@/components/catalog/ProductImage';
import { formatMoney } from '@/lib/products/formatPrice';

export function ProductCardGrid({
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
    <article className="catalog-product-card-grid">
      <button
        type="button"
        className="block w-full text-left focus-visible:outline-none"
        onClick={openDetail}
        disabled={!onOpenDetail}
      >
        <ProductImage product={product} className="aspect-square w-full" />
        <div className="space-y-1 p-3">
          <h3 className="line-clamp-2 text-sm font-semibold text-foreground">{product.name}</h3>
          <p className="text-xs text-muted-foreground">
            {skuLabel}: {product.sku}
          </p>
          <p className="catalog-price catalog-price--prominent">
            {formatMoney(product.priceAmount, product.priceCurrency)}
          </p>
        </div>
      </button>
      <div className="mt-auto p-3 pt-0">
        <Button
          className="w-full"
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

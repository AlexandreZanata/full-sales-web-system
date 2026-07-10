import { ProductCardAddPill } from '@/components/catalog/ProductCardAddPill';
import { ProductCardPrice } from '@/components/catalog/ProductCardPrice';
import { ProductCardTitleRow } from '@/components/catalog/ProductCardTitleRow';
import type { ProductCardProps } from '@/components/catalog/productCardProps';
import { ProductImage } from '@/components/catalog/ProductImage';
import { productCardDescription } from '@/lib/catalog/stripHtml';
import { useI18n } from '@/lib/i18n/context';

export function ProductCardGrid({
  product,
  onAddToCart,
  onOpenDetail,
  addToCartLabel,
}: ProductCardProps) {
  const { t } = useI18n();
  const description = productCardDescription(product.description);

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
        aria-label={product.name}
      >
        <ProductImage product={product} className="catalog-product-card-grid-image" />
      </button>
      <div className="flex flex-1 flex-col gap-2 p-3">
        <ProductCardTitleRow product={product} onOpenDetail={onOpenDetail} />
        {description ? (
          <p className="line-clamp-2 text-xs text-muted-foreground">{description}</p>
        ) : null}
        <div className="mt-auto flex items-center justify-between gap-2 pt-1">
          <ProductCardPrice
            priceAmount={product.priceAmount}
            priceCurrency={product.priceCurrency}
            compareAtPrice={product.compareAtPrice}
          />
          <ProductCardAddPill
            label={t('catalog.addShort')}
            ariaLabel={addToCartLabel}
            onClick={() => {
              onAddToCart(product);
            }}
          />
        </div>
      </div>
    </article>
  );
}

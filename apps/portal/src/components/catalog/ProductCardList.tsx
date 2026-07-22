import { ProductCardAddPill } from '@/components/catalog/ProductCardAddPill';
import { ProductCardPrice } from '@/components/catalog/ProductCardPrice';
import { ProductCardTitleRow } from '@/components/catalog/ProductCardTitleRow';
import type { ProductCardProps } from '@/components/catalog/productCardProps';
import { ProductImage } from '@/components/catalog/ProductImage';
import { productCardDescription } from '@/lib/catalog/stripHtml';
import { useI18n } from '@/lib/i18n/context';
import { cn } from '@/lib/utils';

export function ProductCardList({
  product,
  onAddToCart,
  onOpenDetail,
  addToCartLabel,
}: ProductCardProps) {
  const { t } = useI18n();
  const description = productCardDescription(product.description);
  const canOpen = Boolean(onOpenDetail);

  const openDetail = () => {
    onOpenDetail?.(product);
  };

  return (
    <article
      className={cn('catalog-product-card-list', canOpen && 'cursor-pointer')}
      onClick={canOpen ? openDetail : undefined}
    >
      <ProductImage
        product={product}
        className="size-28 shrink-0 self-start rounded-lg sm:size-32"
      />
      <div className="flex min-w-0 flex-1 flex-col gap-2">
        <ProductCardTitleRow
          product={product}
          titleClassName="text-base font-semibold leading-snug"
          onOpenDetail={canOpen ? openDetail : undefined}
        />
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
            onClick={(event) => {
              event.stopPropagation();
              onAddToCart(product);
            }}
          />
        </div>
      </div>
    </article>
  );
}

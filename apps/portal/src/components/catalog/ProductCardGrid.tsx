import type { KeyboardEvent } from 'react';

import { ProductCardAddPill } from '@/components/catalog/ProductCardAddPill';
import { ProductCardPrice } from '@/components/catalog/ProductCardPrice';
import { ProductCardTitleRow } from '@/components/catalog/ProductCardTitleRow';
import type { ProductCardProps } from '@/components/catalog/productCardProps';
import { ProductImage } from '@/components/catalog/ProductImage';
import { productCardDescription } from '@/lib/catalog/stripHtml';
import { useI18n } from '@/lib/i18n/context';
import { cn } from '@/lib/utils';

export function ProductCardGrid({
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

  const onCardKeyDown = (event: KeyboardEvent<HTMLElement>) => {
    if (!canOpen) {
      return;
    }
    if (event.key === 'Enter' || event.key === ' ') {
      event.preventDefault();
      openDetail();
    }
  };

  return (
    <article
      className={cn('catalog-product-card-grid', canOpen && 'cursor-pointer')}
      onClick={canOpen ? openDetail : undefined}
      onKeyDown={canOpen ? onCardKeyDown : undefined}
      role={canOpen ? 'link' : undefined}
      tabIndex={canOpen ? 0 : undefined}
      aria-label={canOpen ? product.name : undefined}
    >
      <ProductImage product={product} className="catalog-product-card-grid-image" />
      <div className="flex flex-1 flex-col gap-2 p-3">
        <ProductCardTitleRow product={product} />
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

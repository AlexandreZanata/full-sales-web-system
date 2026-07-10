import { Info } from 'lucide-react';
import { useState } from 'react';

import { ProductInfoDialog } from '@/components/catalog/ProductInfoDialog';
import type { PortalProduct } from '@/lib/api/types';
import { productCardDescription } from '@/lib/catalog/stripHtml';
import { useI18n } from '@/lib/i18n/context';

type ProductCardTitleRowProps = {
  product: PortalProduct;
  onOpenDetail?: (product: PortalProduct) => void;
  titleClassName?: string;
};

export function ProductCardTitleRow({
  product,
  onOpenDetail,
  titleClassName = 'text-sm font-semibold',
}: ProductCardTitleRowProps) {
  const { t } = useI18n();
  const [infoOpen, setInfoOpen] = useState(false);
  const hasInfo = Boolean(productCardDescription(product.description));

  const openDetail = () => {
    onOpenDetail?.(product);
  };

  return (
    <>
      <div className="flex items-start justify-between gap-2">
        <h3 className={`min-w-0 flex-1 ${titleClassName}`}>
          <button
            type="button"
            className="w-full text-left focus-visible:outline-none"
            onClick={openDetail}
            disabled={!onOpenDetail}
          >
            <span className="line-clamp-2">{product.name}</span>
          </button>
        </h3>
        {hasInfo ? (
          <button
            type="button"
            className="shrink-0 rounded-full p-1 text-muted-foreground hover:bg-surface-muted hover:text-foreground"
            aria-label={t('productDetail.productInfo')}
            onClick={() => {
              setInfoOpen(true);
            }}
          >
            <Info className="size-4" aria-hidden />
          </button>
        ) : null}
      </div>
      <ProductInfoDialog
        product={product}
        open={infoOpen}
        onClose={() => {
          setInfoOpen(false);
        }}
      />
    </>
  );
}

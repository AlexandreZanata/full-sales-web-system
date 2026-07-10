import { useEffect, useRef } from 'react';

import type { PortalProduct } from '@/lib/api/types';
import { productCardDescription } from '@/lib/catalog/stripHtml';
import { useI18n } from '@/lib/i18n/context';

type ProductInfoDialogProps = {
  product: PortalProduct;
  open: boolean;
  onClose: () => void;
};

export function ProductInfoDialog({ product, open, onClose }: ProductInfoDialogProps) {
  const { t } = useI18n();
  const dialogRef = useRef<HTMLDialogElement>(null);
  const description = productCardDescription(product.description);

  useEffect(() => {
    const dialog = dialogRef.current;
    if (!dialog) {
      return;
    }
    if (open && !dialog.open) {
      dialog.showModal();
      return;
    }
    if (!open && dialog.open) {
      dialog.close();
    }
  }, [open]);

  if (!open) {
    return null;
  }

  return (
    <dialog
      ref={dialogRef}
      className="w-[min(100%,24rem)] rounded-2xl border border-hairline bg-surface p-0 shadow-xl backdrop:bg-foreground/40"
      onClose={onClose}
    >
      <form method="dialog" className="flex flex-col gap-3 p-4">
        <h2 className="text-base font-semibold text-foreground">{product.name}</h2>
        {description ? (
          <p className="text-sm text-muted-foreground">{description}</p>
        ) : (
          <p className="text-sm text-muted-foreground">{t('catalog.noDescription')}</p>
        )}
        <p className="text-xs text-muted-foreground">
          {t('catalog.sku')}: {product.sku}
        </p>
        <button
          type="submit"
          className="mt-1 self-end rounded-lg bg-primary px-3 py-1.5 text-sm font-medium text-primary-foreground"
        >
          {t('common.confirm')}
        </button>
      </form>
    </dialog>
  );
}

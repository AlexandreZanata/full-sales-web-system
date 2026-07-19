import { Link } from '@tanstack/react-router';
import { AlertTriangle } from 'lucide-react';

import { Button } from '@/components/ui/Button';
import { useI18n } from '@/lib/i18n/context';

type SaleInsufficientStockAlertProps = {
  productId: string;
};

export function SaleInsufficientStockAlert({ productId }: SaleInsufficientStockAlertProps) {
  const { t } = useI18n();

  return (
    <div
      role="alert"
      className="mb-4 flex flex-wrap items-center gap-3 rounded-md border border-destructive/30 bg-destructive/5 px-4 py-3 text-sm text-destructive"
    >
      <AlertTriangle className="size-4 shrink-0" aria-hidden />
      <p className="min-w-0 flex-1 font-medium">{t('errors.sales.insufficientStock')}</p>
      <Link to="/inventory/adjustments" search={{ productId }}>
        <Button type="button" variant="secondary" className="min-h-9 border-destructive/30">
          {t('sales.detail.goToStock')}
        </Button>
      </Link>
    </div>
  );
}

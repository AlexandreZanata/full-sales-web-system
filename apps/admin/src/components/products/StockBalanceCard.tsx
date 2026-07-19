import { Link } from '@tanstack/react-router';
import { useQuery } from '@tanstack/react-query';

import { Button } from '@/components/ui/Button';
import { Card } from '@/components/ui/Card';
import { LoadingSpinner } from '@/components/ui/LoadingSpinner';
import { fetchStockBalance } from '@/lib/api/inventory';
import { useI18n } from '@/lib/i18n/context';
import { cn } from '@/lib/utils';

type StockBalanceCardProps = {
  productId: string;
};

export function StockBalanceCard({ productId }: StockBalanceCardProps) {
  const { t } = useI18n();
  const balance = useQuery({
    queryKey: ['inventory', 'balance', productId],
    queryFn: () => fetchStockBalance(productId),
  });

  const available = balance.data?.available;
  const isEmpty = available === 0;

  return (
    <Card className={cn('space-y-3', isEmpty && 'border-destructive/30 bg-destructive/5')}>
      <div className="flex flex-wrap items-start justify-between gap-3">
        <div className="min-w-0 space-y-2">
          <p className="text-xs font-semibold uppercase tracking-[0.12em] text-muted-foreground">
            {t('products.stock.available')}
          </p>
          {balance.isLoading ? (
            <LoadingSpinner />
          ) : balance.data ? (
            <p
              className={cn(
                'text-2xl font-semibold',
                isEmpty ? 'text-destructive' : 'text-foreground',
              )}
            >
              {t('products.stock.units').replace('{count}', String(balance.data.available))}
            </p>
          ) : (
            <p className="text-sm text-muted-foreground">{t('products.stock.loadError')}</p>
          )}
        </div>
        <Link to="/inventory/adjustments" search={{ productId }}>
          <Button type="button" variant={isEmpty ? 'danger' : 'secondary'} className="min-h-9">
            {t('products.stock.goToStock')}
          </Button>
        </Link>
      </div>
      {isEmpty ? <p className="text-sm text-destructive">{t('products.stock.emptyHint')}</p> : null}
    </Card>
  );
}

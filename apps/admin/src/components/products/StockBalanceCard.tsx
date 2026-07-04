import { useQuery } from '@tanstack/react-query';

import { Card } from '@/components/ui/Card';
import { LoadingSpinner } from '@/components/ui/LoadingSpinner';
import { fetchStockBalance } from '@/lib/api/inventory';
import { useI18n } from '@/lib/i18n/context';

type StockBalanceCardProps = {
  productId: string;
};

export function StockBalanceCard({ productId }: StockBalanceCardProps) {
  const { t } = useI18n();
  const balance = useQuery({
    queryKey: ['inventory', 'balance', productId],
    queryFn: () => fetchStockBalance(productId),
  });

  return (
    <Card className="space-y-2">
      <p className="text-xs font-semibold uppercase tracking-[0.12em] text-muted-foreground">
        {t('products.stock.available')}
      </p>
      {balance.isLoading ? (
        <LoadingSpinner />
      ) : balance.data ? (
        <p className="text-2xl font-semibold text-foreground">
          {t('products.stock.units').replace('{count}', String(balance.data.available))}
        </p>
      ) : (
        <p className="text-sm text-muted-foreground">{t('products.stock.loadError')}</p>
      )}
    </Card>
  );
}

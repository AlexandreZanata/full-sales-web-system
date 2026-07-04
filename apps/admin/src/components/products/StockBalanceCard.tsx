import { useQuery } from '@tanstack/react-query';

import { Card } from '@/components/ui/Card';
import { LoadingSpinner } from '@/components/ui/LoadingSpinner';
import { fetchStockBalance } from '@/lib/api/inventory';

type StockBalanceCardProps = {
  productId: string;
};

export function StockBalanceCard({ productId }: StockBalanceCardProps) {
  const balance = useQuery({
    queryKey: ['inventory', 'balance', productId],
    queryFn: () => fetchStockBalance(productId),
  });

  return (
    <Card className="space-y-2">
      <p className="text-xs font-semibold uppercase tracking-[0.12em] text-muted-foreground">
        Available stock
      </p>
      {balance.isLoading ? (
        <LoadingSpinner />
      ) : balance.data ? (
        <p className="text-2xl font-semibold text-foreground">{balance.data.available} units</p>
      ) : (
        <p className="text-sm text-muted-foreground">Unable to load stock balance.</p>
      )}
    </Card>
  );
}

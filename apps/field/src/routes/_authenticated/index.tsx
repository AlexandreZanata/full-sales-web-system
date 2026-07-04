import { useQuery } from '@tanstack/react-query';
import { Link, createFileRoute } from '@tanstack/react-router';

import { SaleStatusBadge } from '@/components/sales/SaleStatusBadge';
import { Button } from '@/components/ui/Button';
import { EmptyState } from '@/components/ui/EmptyState';
import { LoadingSpinner } from '@/components/ui/LoadingSpinner';
import { fetchSales } from '@/lib/api/sales';
import { useI18n } from '@/lib/i18n/context';
import { formatMoney } from '@/lib/products/formatPrice';

export const Route = createFileRoute('/_authenticated/')({
  component: SalesListPage,
});

function SalesListPage() {
  const { t } = useI18n();
  const salesQuery = useQuery({
    queryKey: ['sales'],
    queryFn: () => fetchSales({ pageSize: 20 }),
  });

  if (salesQuery.isLoading) return <LoadingSpinner className="py-16" />;
  if (salesQuery.isError) {
    return (
      <EmptyState
        title={t('common.loadFailed')}
        action={<Button onClick={() => void salesQuery.refetch()}>{t('common.tryAgain')}</Button>}
      />
    );
  }

  const sales = salesQuery.data?.items ?? [];

  return (
    <div className="space-y-4">
      <div className="flex items-center justify-between">
        <h1 className="text-2xl font-semibold">{t('sales.title')}</h1>
        <Link to="/sales/new">
          <Button>{t('sales.new')}</Button>
        </Link>
      </div>
      {sales.length === 0 ? (
        <EmptyState title={t('common.emptySales')} />
      ) : (
        <ul className="divide-y divide-hairline rounded-lg border border-hairline bg-surface">
          {sales.map((sale) => (
            <li key={sale.id}>
              <Link
                to="/sales/$id"
                params={{ id: sale.id }}
                className="flex flex-col gap-2 p-4 hover:bg-surface-muted sm:flex-row sm:items-center sm:justify-between"
              >
                <div>
                  <p className="font-mono text-xs text-muted-foreground">{sale.id.slice(0, 8)}…</p>
                  <p className="text-sm text-muted-foreground">
                    {new Date(sale.createdAt).toLocaleString()}
                  </p>
                </div>
                <div className="flex items-center gap-3">
                  <SaleStatusBadge status={sale.status} />
                  <span className="font-semibold">
                    {formatMoney(sale.totalAmount, sale.totalCurrency)}
                  </span>
                </div>
              </Link>
            </li>
          ))}
        </ul>
      )}
    </div>
  );
}

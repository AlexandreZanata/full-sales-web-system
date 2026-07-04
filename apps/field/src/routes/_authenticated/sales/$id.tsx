import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';
import { Link, createFileRoute } from '@tanstack/react-router';
import { useState } from 'react';

import { SaleStatusBadge } from '@/components/sales/SaleStatusBadge';
import { Button } from '@/components/ui/Button';
import { Card } from '@/components/ui/Card';
import { EmptyState } from '@/components/ui/EmptyState';
import { LoadingSpinner } from '@/components/ui/LoadingSpinner';
import { cancelSale, confirmSale, fetchSale } from '@/lib/api/sales';
import { ApiError } from '@/lib/api/client';
import { useI18n } from '@/lib/i18n/context';
import { saleActionErrorMessage } from '@/lib/sales/constants';
import { formatMoney } from '@/lib/products/formatPrice';

export const Route = createFileRoute('/_authenticated/sales/$id')({
  component: SaleDetailPage,
});

function SaleDetailPage() {
  const { id } = Route.useParams();
  const { t } = useI18n();
  const queryClient = useQueryClient();
  const [error, setError] = useState<string | null>(null);

  const saleQuery = useQuery({
    queryKey: ['sales', id],
    queryFn: () => fetchSale(id),
  });

  const confirmMutation = useMutation({
    mutationFn: () => confirmSale(id),
    onSuccess: async () => {
      await queryClient.invalidateQueries({ queryKey: ['sales'] });
      await saleQuery.refetch();
    },
    onError: (err) => {
      setError(err instanceof ApiError ? saleActionErrorMessage(err.code) : t('common.loadFailed'));
    },
  });

  const cancelMutation = useMutation({
    mutationFn: () => cancelSale(id),
    onSuccess: async () => {
      await queryClient.invalidateQueries({ queryKey: ['sales'] });
      await saleQuery.refetch();
    },
    onError: (err) => {
      setError(err instanceof ApiError ? saleActionErrorMessage(err.code) : t('common.loadFailed'));
    },
  });

  if (saleQuery.isLoading) return <LoadingSpinner className="py-16" />;
  if (saleQuery.isError || !saleQuery.data) {
    return <EmptyState title={t('common.loadFailed')} />;
  }

  const sale = saleQuery.data;
  const isPending = sale.status === 'Pending';

  return (
    <div className="space-y-4 pb-24">
      <Link to="/" className="text-sm text-muted-foreground hover:text-foreground">
        ← {t('sales.title')}
      </Link>
      <div className="flex items-center justify-between gap-3">
        <h1 className="text-2xl font-semibold">{t('sales.detail')}</h1>
        <SaleStatusBadge status={sale.status} />
      </div>

      <Card className="space-y-4">
        <div className="flex justify-between text-sm font-semibold">
          <span>{t('common.total')}</span>
          <span>{formatMoney(sale.totalAmount, sale.totalCurrency)}</span>
        </div>
        <ul className="divide-y divide-hairline border-y border-hairline">
          {sale.items.map((item) => (
            <li key={item.productId} className="flex justify-between py-3 text-sm">
              <span>
                {item.quantity}× {item.productId.slice(0, 8)}
              </span>
              <span>{formatMoney(item.lineTotalAmount, item.unitPriceCurrency)}</span>
            </li>
          ))}
        </ul>
        {error ? <p className="text-sm text-destructive">{error}</p> : null}
      </Card>

      {isPending ? (
        <div className="fixed inset-x-0 bottom-16 flex gap-2 border-t border-hairline bg-surface p-4 md:static md:border-0 md:bg-transparent md:p-0">
          <Button
            className="flex-1 md:flex-none"
            disabled={confirmMutation.isPending}
            onClick={() => {
              setError(null);
              confirmMutation.mutate();
            }}
          >
            {confirmMutation.isPending ? t('common.working') : t('sales.confirm')}
          </Button>
          <Button
            variant="danger"
            className="flex-1 md:flex-none"
            disabled={cancelMutation.isPending}
            onClick={() => {
              setError(null);
              cancelMutation.mutate();
            }}
          >
            {cancelMutation.isPending ? t('common.working') : t('sales.cancel')}
          </Button>
        </div>
      ) : null}
    </div>
  );
}

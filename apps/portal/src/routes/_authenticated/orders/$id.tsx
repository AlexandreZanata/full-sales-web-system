import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';
import { Link, createFileRoute } from '@tanstack/react-router';

import { OrderStatusBadge } from '@/components/orders/OrderStatusBadge';
import { OrderStatusTimeline } from '@/components/orders/OrderStatusTimeline';
import { Button } from '@/components/ui/Button';
import { Card } from '@/components/ui/Card';
import { EmptyState } from '@/components/ui/EmptyState';
import { LoadingSpinner } from '@/components/ui/LoadingSpinner';
import { deletePortalOrder, fetchPortalOrder, submitPortalOrder } from '@/lib/api/portal';
import { useI18n } from '@/lib/i18n/context';
import { formatMoney } from '@/lib/products/formatPrice';

export const Route = createFileRoute('/_authenticated/orders/$id')({
  component: OrderDetailPage,
});

function OrderDetailPage() {
  const { id } = Route.useParams();
  const { t } = useI18n();
  const queryClient = useQueryClient();

  const orderQuery = useQuery({
    queryKey: ['portal', 'orders', id],
    queryFn: () => fetchPortalOrder(id),
  });

  const submitMutation = useMutation({
    mutationFn: () => submitPortalOrder(id),
    onSuccess: async () => {
      await queryClient.invalidateQueries({ queryKey: ['portal', 'orders'] });
      await orderQuery.refetch();
    },
  });

  const cancelMutation = useMutation({
    mutationFn: () => deletePortalOrder(id),
    onSuccess: async () => {
      await queryClient.invalidateQueries({ queryKey: ['portal', 'orders'] });
      await orderQuery.refetch();
    },
  });

  if (orderQuery.isLoading) {
    return <LoadingSpinner className="py-16" />;
  }

  if (orderQuery.isError || !orderQuery.data) {
    return <EmptyState title={t('common.error.loadFailed')} />;
  }

  const order = orderQuery.data;
  const isDraft = order.status === 'Draft';

  return (
    <div className="mx-auto max-w-2xl space-y-4">
      <Link to="/orders" className="text-sm text-muted-foreground hover:text-foreground">
        ← {t('orders.title')}
      </Link>

      <div className="flex flex-wrap items-center justify-between gap-3">
        <h1 className="text-2xl font-semibold text-foreground">{t('orders.detail')}</h1>
        <OrderStatusBadge status={order.status} />
      </div>

      <Card className="space-y-4">
        <div className="flex justify-between text-sm">
          <span className="text-muted-foreground">{t('common.total')}</span>
          <span className="font-semibold">
            {formatMoney(order.totalAmount, order.totalCurrency)}
          </span>
        </div>

        {order.notes ? <p className="text-sm text-muted-foreground">{order.notes}</p> : null}

        {order.rejectionReason ? (
          <p className="rounded-md border border-destructive/30 bg-destructive/5 px-3 py-2 text-sm text-destructive">
            {t('orders.rejectionReason')}: {order.rejectionReason}
          </p>
        ) : null}

        <ul className="divide-y divide-hairline border-y border-hairline">
          {order.items.map((item) => (
            <li key={item.id} className="flex justify-between py-3 text-sm">
              <span>
                {item.quantity}×{' '}
                <span className="font-mono text-xs">{item.productId.slice(0, 8)}</span>
              </span>
              <span>{formatMoney(item.lineTotalAmount, item.unitPriceCurrency)}</span>
            </li>
          ))}
        </ul>

        <OrderStatusTimeline status={order.status} />

        {isDraft ? (
          <div className="flex flex-wrap gap-2">
            <Button
              disabled={submitMutation.isPending || order.items.length === 0}
              onClick={() => {
                submitMutation.mutate();
              }}
            >
              {submitMutation.isPending ? t('common.working') : t('orders.submit')}
            </Button>
            <Button
              variant="danger"
              disabled={cancelMutation.isPending}
              onClick={() => {
                cancelMutation.mutate();
              }}
            >
              {cancelMutation.isPending ? t('common.working') : t('orders.cancelDraft')}
            </Button>
          </div>
        ) : null}
      </Card>
    </div>
  );
}

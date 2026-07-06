import { useQuery } from '@tanstack/react-query';
import { Link, createFileRoute } from '@tanstack/react-router';

import { usePortalAuth } from '@/auth/usePortalAuth';
import { OrderStatusBadge } from '@/components/orders/OrderStatusBadge';
import { Button } from '@/components/ui/Button';
import { EmptyState } from '@/components/ui/EmptyState';
import { LoadingSpinner } from '@/components/ui/LoadingSpinner';
import { fetchPortalOrders } from '@/lib/api/portal';
import { setPostLoginRedirect } from '@/lib/auth/postLoginRedirect';
import { useI18n } from '@/lib/i18n/context';
import { formatMoney } from '@/lib/products/formatPrice';

export const Route = createFileRoute('/_authenticated/orders/')({
  component: OrdersPage,
});

function OrdersPage() {
  const { t } = useI18n();
  const { user } = usePortalAuth();

  const ordersQuery = useQuery({
    queryKey: ['portal', 'orders'],
    queryFn: () => fetchPortalOrders({ limit: 50 }),
    enabled: Boolean(user),
  });

  if (!user) {
    return (
      <EmptyState
        title={t('orders.guestTitle')}
        description={t('orders.guestDescription')}
        action={
          <Link
            to="/login"
            search={{ redirect: '/orders' }}
            onClick={() => {
              setPostLoginRedirect('/orders');
            }}
          >
            <Button>{t('auth.signIn')}</Button>
          </Link>
        }
      />
    );
  }

  if (ordersQuery.isLoading) {
    return <LoadingSpinner className="py-16" />;
  }

  if (ordersQuery.isError) {
    return (
      <EmptyState
        title={t('common.error.loadFailed')}
        action={<Button onClick={() => void ordersQuery.refetch()}>{t('common.tryAgain')}</Button>}
      />
    );
  }

  const orders = ordersQuery.data?.data ?? [];

  return (
    <div className="space-y-4">
      <h1 className="text-2xl font-semibold text-foreground">{t('orders.title')}</h1>

      {orders.length === 0 ? (
        <EmptyState title={t('common.empty.orders')} />
      ) : (
        <ul className="divide-y divide-hairline overflow-hidden rounded-lg border border-hairline bg-surface">
          {orders.map((order) => (
            <li key={order.id}>
              <Link
                to="/orders/$id"
                params={{ id: order.id }}
                className="flex flex-col gap-2 p-4 transition-colors hover:bg-surface-muted sm:flex-row sm:items-center sm:justify-between"
              >
                <div>
                  <p className="font-mono text-xs text-muted-foreground">{order.id.slice(0, 8)}…</p>
                  <p className="text-sm text-muted-foreground">
                    {new Date(order.createdAt).toLocaleString()}
                  </p>
                </div>
                <div className="flex items-center gap-3">
                  <OrderStatusBadge status={order.status} />
                  <span className="text-sm font-semibold">
                    {formatMoney(order.totalAmount, order.totalCurrency)}
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

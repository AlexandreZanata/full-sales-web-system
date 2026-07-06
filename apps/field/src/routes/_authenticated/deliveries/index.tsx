import { useQuery } from '@tanstack/react-query';
import { Link, createFileRoute } from '@tanstack/react-router';

import { DeliveryStatusBadge } from '@/components/deliveries/DeliveryStatusBadge';
import { Card } from '@/components/ui/Card';
import { EmptyState } from '@/components/ui/EmptyState';
import { LoadingSpinner } from '@/components/ui/LoadingSpinner';
import { fetchDeliveries } from '@/lib/api/deliveries';
import { useI18n } from '@/lib/i18n/context';

export const Route = createFileRoute('/_authenticated/deliveries/')({
  component: DeliveriesPage,
});

function DeliveriesPage() {
  const { t } = useI18n();
  const deliveriesQuery = useQuery({
    queryKey: ['deliveries'],
    queryFn: () => fetchDeliveries(),
  });

  if (deliveriesQuery.isLoading) {
    return <LoadingSpinner className="py-16" />;
  }

  if (deliveriesQuery.isError) {
    return <EmptyState title={t('common.loadFailed')} />;
  }

  const items = deliveriesQuery.data?.data ?? [];

  if (items.length === 0) {
    return <EmptyState title={t('deliveries.empty')} />;
  }

  return (
    <div className="space-y-4">
      <h1 className="text-2xl font-semibold">{t('deliveries.title')}</h1>
      <ul className="space-y-3">
        {items.map((delivery) => (
          <li key={delivery.id}>
            <Link to="/deliveries/$id" params={{ id: delivery.id }}>
              <Card className="flex items-center justify-between gap-3 transition-colors hover:bg-surface-muted">
                <div>
                  <p className="text-sm font-medium">
                    {t('deliveries.orderId')}{' '}
                    <span className="font-mono text-xs">{delivery.orderId.slice(0, 8)}</span>
                  </p>
                </div>
                <DeliveryStatusBadge status={delivery.status} />
              </Card>
            </Link>
          </li>
        ))}
      </ul>
    </div>
  );
}

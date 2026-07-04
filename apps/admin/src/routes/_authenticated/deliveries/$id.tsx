import { Link, createFileRoute } from '@tanstack/react-router';
import { useQuery } from '@tanstack/react-query';
import { type ReactNode } from 'react';

import { DeliveryStatusTimeline } from '@/components/deliveries/DeliveryStatusTimeline';
import { DomainStatusBadge } from '@/components/status/DomainStatusBadge';
import { Card } from '@/components/ui/Card';
import { LoadingSpinner } from '@/components/ui/LoadingSpinner';
import { PageBackLink } from '@/components/ui/PageBackLink';
import { PageHeader } from '@/components/ui/PageHeader';
import { fetchDelivery } from '@/lib/api/deliveries';
import { fetchUser } from '@/lib/api/users';
import { getDeliveryStatusToken, type DeliveryStatus } from '@/lib/admin-tokens';
import { useI18n } from '@/lib/i18n/context';
import { translateDeliveryStatus } from '@/lib/i18n/labels';

export const Route = createFileRoute('/_authenticated/deliveries/$id')({
  component: DeliveryDetailPage,
});

function DeliveryDetailPage() {
  const { id } = Route.useParams();
  const { t } = useI18n();

  const delivery = useQuery({
    queryKey: ['deliveries', id],
    queryFn: () => fetchDelivery(id),
  });

  const driver = useQuery({
    queryKey: ['users', delivery.data?.driverId],
    queryFn: () => {
      const driverId = delivery.data?.driverId;
      if (!driverId) {
        throw new Error('Driver id missing');
      }
      return fetchUser(driverId);
    },
    enabled: Boolean(delivery.data?.driverId),
  });

  if (delivery.isLoading) {
    return (
      <div className="flex justify-center py-16">
        <LoadingSpinner />
      </div>
    );
  }

  if (!delivery.data) {
    return (
      <PageHeader
        title={t('deliveries.detail.notFound')}
        back={<PageBackLink label={t('common.backTo.deliveries')} to="/deliveries" />}
      />
    );
  }

  const detail = delivery.data;

  return (
    <div className="space-y-6">
      <PageHeader
        title={`${t('forms.fields.delivery')} ${detail.id.slice(0, 8)}…`}
        description={t('deliveries.detail.description')}
        back={<PageBackLink label={t('common.backTo.deliveries')} to="/deliveries" />}
      />

      <Card className="space-y-3 p-5">
        <DetailRow
          label={t('forms.fields.status')}
          value={
            <DomainStatusBadge
              colors={getDeliveryStatusToken(detail.status as DeliveryStatus)}
              label={translateDeliveryStatus(t, detail.status as DeliveryStatus)}
            />
          }
        />
        <DetailRow
          label={t('forms.fields.order')}
          value={
            <Link
              to="/orders/$id"
              params={{ id: detail.orderId }}
              className="font-mono text-xs hover:underline"
            >
              {detail.orderId.slice(0, 8)}…
            </Link>
          }
        />
        <DetailRow
          label={t('forms.fields.driver')}
          value={
            driver.data ? (
              <Link to="/users/$id" params={{ id: detail.driverId }} className="hover:underline">
                {driver.data.name}
              </Link>
            ) : (
              detail.driverId
            )
          }
        />
        {detail.saleId ? (
          <DetailRow
            label={t('forms.fields.sale')}
            value={
              <Link to="/sales" className="font-mono text-xs hover:underline">
                {detail.saleId.slice(0, 8)}…
              </Link>
            }
          />
        ) : null}
      </Card>

      <DeliveryStatusTimeline status={detail.status} />
    </div>
  );
}

function DetailRow({ label, value }: { label: string; value: ReactNode }) {
  return (
    <div className="flex flex-col gap-1 sm:flex-row sm:items-center sm:justify-between">
      <span className="text-xs font-semibold uppercase tracking-[0.12em] text-muted-foreground">
        {label}
      </span>
      <span className="text-sm text-foreground">{value}</span>
    </div>
  );
}

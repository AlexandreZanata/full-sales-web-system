import { Link } from '@tanstack/react-router';

import { OrderStatusBadge } from '@/components/orders/OrderStatusBadge';
import { DomainStatusBadge } from '@/components/status/DomainStatusBadge';
import {
  DetailField,
  DetailFieldGrid,
  DetailSectionCard,
  DetailSummaryCard,
} from '@/components/ui/DetailFields';
import type { OrderDetail } from '@/lib/api/types';
import { getDeliveryStatusToken, type DeliveryStatus } from '@/lib/admin-tokens';
import { useI18n } from '@/lib/i18n/context';
import { translateDeliveryStatus } from '@/lib/i18n/labels';
import { formatMoney } from '@/lib/products/formatPrice';

type OrderDetailSummaryProps = {
  detail: OrderDetail;
  commerceName?: string;
  commerceReady: boolean;
};

export function OrderDetailSummary({
  detail,
  commerceName,
  commerceReady,
}: OrderDetailSummaryProps) {
  const { t } = useI18n();

  return (
    <>
      <DetailSummaryCard
        status={<OrderStatusBadge status={detail.status} />}
        subtitle={commerceName}
        totalLabel={t('common.table.total')}
        totalValue={formatMoney(detail.totalAmount, detail.totalCurrency)}
      >
        <DetailFieldGrid>
          <DetailField
            label={t('forms.fields.commerce')}
            value={
              commerceReady ? (
                <Link
                  to="/commerces/$id"
                  params={{ id: detail.commerceId }}
                  className="font-medium hover:underline"
                >
                  {commerceName}
                </Link>
              ) : (
                detail.commerceId
              )
            }
          />
          {detail.notes ? (
            <DetailField label={t('forms.fields.notes')} value={detail.notes} />
          ) : null}
          {detail.rejectionReason ? (
            <DetailField label={t('forms.fields.rejectionReason')} value={detail.rejectionReason} />
          ) : null}
        </DetailFieldGrid>
      </DetailSummaryCard>

      {detail.delivery ? (
        <DetailSectionCard title={t('orders.detail.deliverySection')}>
          <div className="p-5">
            <DetailFieldGrid>
              <DetailField
                label={t('forms.fields.delivery')}
                value={
                  <Link
                    to="/deliveries/$id"
                    params={{ id: detail.delivery.id }}
                    className="font-mono text-xs font-medium hover:underline"
                  >
                    {detail.delivery.id.slice(0, 8)}…
                  </Link>
                }
              />
              <DetailField
                label={t('forms.fields.status')}
                value={
                  <DomainStatusBadge
                    colors={getDeliveryStatusToken(detail.delivery.status as DeliveryStatus)}
                    label={translateDeliveryStatus(t, detail.delivery.status as DeliveryStatus)}
                  />
                }
              />
            </DetailFieldGrid>
          </div>
        </DetailSectionCard>
      ) : null}
    </>
  );
}

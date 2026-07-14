import { Link } from '@tanstack/react-router';

import { DeclaredPaymentBadge } from '@/components/sales/DeclaredPaymentBadge';
import { PaymentMethodBadge } from '@/components/sales/PaymentMethodBadge';
import { SaleStatusBadge } from '@/components/sales/SaleStatusBadge';
import { DetailField, DetailFieldGrid, DetailSummaryCard } from '@/components/ui/DetailFields';
import type { SaleDetail } from '@/lib/api/types';
import { useI18n } from '@/lib/i18n/context';
import { formatMoney } from '@/lib/products/formatPrice';

type SaleDetailSummaryProps = {
  detail: SaleDetail;
  commerceName?: string;
  commerceReady: boolean;
  driverName?: string;
  driverReady: boolean;
};

export function SaleDetailSummary({
  detail,
  commerceName,
  commerceReady,
  driverName,
  driverReady,
}: SaleDetailSummaryProps) {
  const { t } = useI18n();

  return (
    <DetailSummaryCard
      status={<SaleStatusBadge status={detail.status} />}
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
        <DetailField
          label={t('forms.fields.driver')}
          value={
            driverReady ? (
              <Link
                to="/users/$id"
                params={{ id: detail.driverId }}
                className="font-medium hover:underline"
              >
                {driverName}
              </Link>
            ) : (
              detail.driverId
            )
          }
        />
        {detail.orderId ? (
          <DetailField
            label={t('forms.fields.order')}
            value={
              <Link
                to="/orders/$id"
                params={{ id: detail.orderId }}
                className="font-mono text-xs font-medium hover:underline"
              >
                {detail.orderId.slice(0, 8)}…
              </Link>
            }
          />
        ) : null}
        <DetailField
          label={t('forms.fields.paymentMethod')}
          value={<PaymentMethodBadge method={detail.paymentMethod} />}
        />
        <DetailField
          label={t('forms.fields.declaredPayment')}
          value={
            <DeclaredPaymentBadge
              method={detail.declaredPaymentMethod}
              received={detail.declaredPaymentReceived}
            />
          }
        />
      </DetailFieldGrid>
    </DetailSummaryCard>
  );
}

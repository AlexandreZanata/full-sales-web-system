import { DomainStatusBadge } from '@/components/status/DomainStatusBadge';
import { getOrderStatusToken, type OrderStatus } from '@/lib/admin-tokens';
import { useI18n } from '@/lib/i18n/context';
import { translateOrderStatus } from '@/lib/i18n/labels';

type OrderStatusBadgeProps = {
  status: string;
};

export function OrderStatusBadge({ status }: OrderStatusBadgeProps) {
  const { t } = useI18n();
  const token = getOrderStatusToken(status as OrderStatus);
  return (
    <DomainStatusBadge colors={token} label={translateOrderStatus(t, status as OrderStatus)} />
  );
}

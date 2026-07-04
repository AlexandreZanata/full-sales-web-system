import { DomainStatusBadge } from '@/components/status/DomainStatusBadge';
import { getOrderStatusToken } from '@/lib/client-tokens';
import { useI18n } from '@/lib/i18n/context';

type OrderStatusBadgeProps = {
  status: string;
  className?: string;
};

export function OrderStatusBadge({ status, className }: OrderStatusBadgeProps) {
  const { orderStatus } = useI18n();
  const token = getOrderStatusToken(status);

  return <DomainStatusBadge colors={token} label={orderStatus(status)} className={className} />;
}

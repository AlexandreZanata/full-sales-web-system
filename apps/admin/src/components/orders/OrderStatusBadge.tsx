import { DomainStatusBadge } from '@/components/status/DomainStatusBadge';
import { getOrderStatusToken, type OrderStatus } from '@/lib/admin-tokens';

type OrderStatusBadgeProps = {
  status: string;
};

export function OrderStatusBadge({ status }: OrderStatusBadgeProps) {
  const token = getOrderStatusToken(status as OrderStatus);
  return <DomainStatusBadge colors={token} />;
}

import { DomainStatusBadge } from '@/components/status/DomainStatusBadge';
import { getSaleStatusToken, type SaleStatus } from '@/lib/admin-tokens';

type SaleStatusBadgeProps = {
  status: string;
};

export function SaleStatusBadge({ status }: SaleStatusBadgeProps) {
  const token = getSaleStatusToken(status as SaleStatus);
  return <DomainStatusBadge colors={token} />;
}

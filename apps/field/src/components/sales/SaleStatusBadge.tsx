import { DomainStatusBadge } from '@/components/status/DomainStatusBadge';
import { getSaleStatusToken } from '@/lib/client-tokens';
import { useI18n } from '@/lib/i18n/context';

type SaleStatusBadgeProps = { status: string; className?: string };

export function SaleStatusBadge({ status, className }: SaleStatusBadgeProps) {
  const { saleStatus } = useI18n();
  const token = getSaleStatusToken(status);

  return <DomainStatusBadge colors={token} label={saleStatus(status)} className={className} />;
}

import { DomainStatusBadge } from '@/components/status/DomainStatusBadge';
import { getSaleStatusToken, type SaleStatus } from '@/lib/admin-tokens';
import { useI18n } from '@/lib/i18n/context';
import { translateSaleStatus } from '@/lib/i18n/labels';

type SaleStatusBadgeProps = {
  status: string;
};

export function SaleStatusBadge({ status }: SaleStatusBadgeProps) {
  const { t } = useI18n();
  const token = getSaleStatusToken(status as SaleStatus);
  return <DomainStatusBadge colors={token} label={translateSaleStatus(t, status as SaleStatus)} />;
}

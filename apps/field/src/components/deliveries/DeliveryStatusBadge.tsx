import { DomainStatusBadge } from '@/components/status/DomainStatusBadge';
import { getDeliveryStatusToken } from '@/lib/client-tokens';
import { useI18n } from '@/lib/i18n/context';

type DeliveryStatusBadgeProps = { status: string; className?: string };

export function DeliveryStatusBadge({ status, className }: DeliveryStatusBadgeProps) {
  const { deliveryStatus } = useI18n();
  const token = getDeliveryStatusToken(status);

  return <DomainStatusBadge colors={token} label={deliveryStatus(status)} className={className} />;
}

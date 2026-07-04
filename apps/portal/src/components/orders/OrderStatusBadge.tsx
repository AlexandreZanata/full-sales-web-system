import { useI18n } from '@/lib/i18n/context';
import { cn } from '@/lib/utils';

type OrderStatusBadgeProps = {
  status: string;
  className?: string;
};

export function OrderStatusBadge({ status, className }: OrderStatusBadgeProps) {
  const { orderStatus } = useI18n();

  return (
    <span
      className={cn(
        'inline-flex items-center gap-1.5 rounded-full border border-hairline bg-surface px-2.5 py-1 text-xs font-medium text-foreground',
        className,
      )}
    >
      <span className="size-1.5 rounded-full bg-accent" aria-hidden />
      {orderStatus(status)}
    </span>
  );
}

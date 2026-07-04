import { SALE_STATUS_LABELS } from '@/lib/sales/constants';
import { cn } from '@/lib/utils';

type SaleStatusBadgeProps = { status: string; className?: string };

export function SaleStatusBadge({ status, className }: SaleStatusBadgeProps) {
  const label =
    status in SALE_STATUS_LABELS
      ? SALE_STATUS_LABELS[status as keyof typeof SALE_STATUS_LABELS]
      : status;

  return (
    <span
      className={cn(
        'inline-flex items-center gap-1.5 rounded-full border border-hairline px-2.5 py-1 text-xs font-medium',
        className,
      )}
    >
      <span className="size-1.5 rounded-full bg-accent" aria-hidden />
      {label}
    </span>
  );
}

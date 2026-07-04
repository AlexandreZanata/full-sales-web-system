import type { StatusToken } from '@/lib/admin-tokens';
import { cn } from '@/lib/utils';

type DomainStatusBadgeProps = {
  colors: StatusToken;
  label: string;
  className?: string;
};

export function DomainStatusBadge({ colors, label, className }: DomainStatusBadgeProps) {
  return (
    <span
      className={cn(
        'inline-flex items-center gap-1.5 rounded-full border px-2.5 py-0.5 text-xs font-medium',
        colors.badge,
        className,
      )}
    >
      <span className={cn('size-1.5 rounded-full', colors.dot)} aria-hidden />
      {label}
    </span>
  );
}

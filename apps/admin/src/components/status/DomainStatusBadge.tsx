import type { StatusToken } from '@/lib/admin-tokens';
import { cn } from '@/lib/utils';

type DomainStatusBadgeProps = {
  colors: StatusToken;
  className?: string;
};

export function DomainStatusBadge({ colors, className }: DomainStatusBadgeProps) {
  return (
    <span
      className={cn(
        'inline-flex items-center gap-1.5 rounded-full border px-2.5 py-0.5 text-xs font-medium',
        colors.badge,
        className,
      )}
    >
      <span className={cn('size-1.5 rounded-full', colors.dot)} aria-hidden />
      {colors.label}
    </span>
  );
}

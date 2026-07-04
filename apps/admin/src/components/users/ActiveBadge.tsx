import { cn } from '@/lib/utils';

type ActiveBadgeProps = {
  active: boolean;
};

export function ActiveBadge({ active }: ActiveBadgeProps) {
  return (
    <span
      className={cn(
        'inline-flex rounded-full border px-2.5 py-0.5 text-xs font-medium',
        active
          ? 'border-status-active/30 text-status-active'
          : 'border-hairline text-muted-foreground',
      )}
    >
      {active ? 'Active' : 'Inactive'}
    </span>
  );
}

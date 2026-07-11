import type { LucideIcon } from 'lucide-react';

import { Card } from '@/components/ui/Card';
import { cn } from '@/lib/utils';

type KpiStatCardProps = {
  label: string;
  value: number | string;
  icon: LucideIcon;
  accentClass: string;
  valueClass?: string;
};

export function KpiStatCard({
  label,
  value,
  icon: Icon,
  accentClass,
  valueClass,
}: KpiStatCardProps) {
  return (
    <Card className="relative overflow-hidden p-4">
      <div className={cn('absolute inset-x-0 top-0 h-1', accentClass)} />
      <div className="flex items-start justify-between gap-3">
        <div className="min-w-0">
          <p className="text-xs font-medium uppercase tracking-wide text-muted-foreground">
            {label}
          </p>
          <p className={cn('mt-2 truncate text-2xl font-semibold tabular-nums', valueClass)}>
            {value}
          </p>
        </div>
        <div
          className={cn(
            'flex size-10 shrink-0 items-center justify-center rounded-lg bg-surface-muted',
            valueClass,
          )}
        >
          <Icon className="size-5 opacity-80" aria-hidden />
        </div>
      </div>
    </Card>
  );
}

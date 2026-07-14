import { type ReactNode } from 'react';

import { adminTokens } from '@/lib/admin-tokens';
import { cn } from '@/lib/utils';

type DetailFieldProps = {
  label: string;
  value: ReactNode;
  emphasize?: boolean;
};

export function DetailField({ label, value, emphasize = false }: DetailFieldProps) {
  return (
    <div className="min-w-0 space-y-1.5">
      <dt className={adminTokens.label}>{label}</dt>
      <dd
        className={cn(
          'text-sm text-foreground',
          emphasize && 'text-xl font-semibold tracking-tight tabular-nums',
        )}
      >
        {value}
      </dd>
    </div>
  );
}

type DetailFieldGridProps = {
  children: ReactNode;
  className?: string;
};

export function DetailFieldGrid({ children, className }: DetailFieldGridProps) {
  return (
    <dl className={cn('grid gap-x-6 gap-y-5 sm:grid-cols-2 lg:grid-cols-3', className)}>
      {children}
    </dl>
  );
}

type DetailSummaryCardProps = {
  status: ReactNode;
  subtitle?: ReactNode;
  totalLabel: string;
  totalValue: string;
  children: ReactNode;
};

export function DetailSummaryCard({
  status,
  subtitle,
  totalLabel,
  totalValue,
  children,
}: DetailSummaryCardProps) {
  return (
    <CardShell>
      <div className="flex flex-wrap items-center justify-between gap-4 border-b border-hairline bg-surface-muted/60 px-5 py-4">
        <div className="flex min-w-0 flex-wrap items-center gap-3">
          {status}
          {subtitle ? (
            <span className="truncate text-sm text-muted-foreground">{subtitle}</span>
          ) : null}
        </div>
        <div className="text-right">
          <p className={adminTokens.label}>{totalLabel}</p>
          <p className="mt-0.5 text-xl font-semibold tracking-tight tabular-nums text-foreground">
            {totalValue}
          </p>
        </div>
      </div>
      <div className="p-5">{children}</div>
    </CardShell>
  );
}

type DetailSectionCardProps = {
  title: string;
  children: ReactNode;
};

export function DetailSectionCard({ title, children }: DetailSectionCardProps) {
  return (
    <CardShell>
      <div className="border-b border-hairline px-5 py-3">
        <h2 className={adminTokens.sectionTitle}>{title}</h2>
      </div>
      <div className="p-0">{children}</div>
    </CardShell>
  );
}

function CardShell({ children }: { children: ReactNode }) {
  return (
    <section className="overflow-hidden rounded-lg border border-hairline bg-surface">
      {children}
    </section>
  );
}

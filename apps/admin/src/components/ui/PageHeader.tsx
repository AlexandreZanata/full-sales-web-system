import { adminTokens } from '@/lib/admin-tokens';
import { cn } from '@/lib/utils';
import { type ReactNode } from 'react';

type PageHeaderProps = {
  title: string;
  description?: string;
  back?: ReactNode;
  actions?: ReactNode;
  className?: string;
};

export function PageHeader({ title, description, back, actions, className }: PageHeaderProps) {
  const rightSlot =
    back || actions ? (
      <div className="flex shrink-0 flex-wrap items-center justify-end gap-2">
        {back}
        {actions}
      </div>
    ) : null;

  return (
    <div className={cn('mb-6', className)}>
      <div className="flex flex-wrap items-start justify-between gap-4">
        <div className="min-w-0">
          <h1 className={adminTokens.pageTitle}>{title}</h1>
          {description ? <p className="mt-1 text-sm text-muted-foreground">{description}</p> : null}
        </div>
        {rightSlot}
      </div>
    </div>
  );
}

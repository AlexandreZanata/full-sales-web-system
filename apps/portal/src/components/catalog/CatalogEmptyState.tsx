import { ShoppingBag } from 'lucide-react';
import { type ReactNode } from 'react';

import { EmptyState } from '@/components/ui/EmptyState';

type CatalogEmptyStateProps = {
  title: string;
  description?: string;
  action?: ReactNode;
};

export function CatalogEmptyState({ title, description, action }: CatalogEmptyStateProps) {
  return (
    <div className="flex flex-col items-center">
      <div className="mb-4 flex size-20 items-center justify-center rounded-full bg-surface-muted text-muted-foreground">
        <ShoppingBag className="size-10 opacity-60" strokeWidth={1.25} aria-hidden />
      </div>
      <EmptyState title={title} description={description} action={action} />
    </div>
  );
}

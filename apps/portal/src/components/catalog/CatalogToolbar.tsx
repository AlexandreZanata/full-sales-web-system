import { LayoutGrid, LayoutList } from 'lucide-react';
import { type ReactNode } from 'react';

import { Button } from '@/components/ui/Button';
import type { CatalogViewMode } from '@/lib/catalog/viewMode';
import { cn } from '@/lib/utils';

type CatalogToolbarProps = {
  title?: string;
  viewMode: CatalogViewMode;
  onViewModeChange: (mode: CatalogViewMode) => void;
  listViewLabel: string;
  gridViewLabel: string;
  searchSlot?: ReactNode;
  className?: string;
};

export function CatalogToolbar({
  title,
  viewMode,
  onViewModeChange,
  listViewLabel,
  gridViewLabel,
  searchSlot,
  className,
}: CatalogToolbarProps) {
  return (
    <div
      className={cn('flex flex-col gap-3 sm:flex-row sm:items-end sm:justify-between', className)}
    >
      <div className="flex min-w-0 flex-1 flex-col gap-3 sm:flex-row sm:items-end sm:justify-between">
        {title ? <h2 className="text-xl font-semibold text-foreground">{title}</h2> : null}
        {searchSlot}
      </div>
      <div className="flex shrink-0 items-center gap-1 rounded-md border border-hairline bg-surface p-1">
        <Button
          type="button"
          variant={viewMode === 'list' ? 'primary' : 'ghost'}
          className="h-9 min-h-9 px-3"
          aria-label={listViewLabel}
          aria-pressed={viewMode === 'list'}
          onClick={() => {
            onViewModeChange('list');
          }}
        >
          <LayoutList className="size-4" aria-hidden />
        </Button>
        <Button
          type="button"
          variant={viewMode === 'grid' ? 'primary' : 'ghost'}
          className="h-9 min-h-9 px-3"
          aria-label={gridViewLabel}
          aria-pressed={viewMode === 'grid'}
          onClick={() => {
            onViewModeChange('grid');
          }}
        >
          <LayoutGrid className="size-4" aria-hidden />
        </Button>
      </div>
    </div>
  );
}

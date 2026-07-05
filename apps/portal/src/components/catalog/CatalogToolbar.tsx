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
      className={cn(
        'flex flex-col gap-3 sm:flex-row sm:items-end sm:justify-between',
        'md:items-center md:gap-4 md:rounded-2xl md:border md:border-hairline/80 md:bg-surface md:p-2.5 md:shadow-sm',
        className,
      )}
    >
      {title ? (
        <h2 className="text-xl font-semibold text-foreground md:shrink-0 md:px-2 md:text-base md:tracking-tight">
          {title}
        </h2>
      ) : null}
      {title && searchSlot ? (
        <div className="hidden h-8 w-px shrink-0 bg-hairline md:block" aria-hidden />
      ) : null}
      {searchSlot ? (
        <div className="min-w-0 sm:flex-1 md:flex md:justify-center">{searchSlot}</div>
      ) : null}
      <div
        className="flex shrink-0 items-center gap-0.5 self-end rounded-md border border-hairline bg-surface p-1 sm:self-auto md:rounded-full md:p-0.5"
        role="group"
        aria-label={gridViewLabel}
      >
        <Button
          type="button"
          variant={viewMode === 'list' ? 'primary' : 'ghost'}
          className="h-9 min-h-9 rounded-full px-3"
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
          className="h-9 min-h-9 rounded-full px-3"
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

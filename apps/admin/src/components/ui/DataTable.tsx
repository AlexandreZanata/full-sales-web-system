import { type ReactNode } from 'react';

import { Button } from '@/components/ui/Button';
import { formatTablePaginationSummary, type TablePaginationState } from '@/lib/tablePagination';
import { cn } from '@/lib/utils';

export type DataTableColumn<T> = {
  id: string;
  header: ReactNode;
  cell: (row: T) => ReactNode;
  headerClassName?: string;
  cellClassName?: string;
  align?: 'left' | 'right';
};

type DataTableProps<T> = {
  caption?: string;
  columns: DataTableColumn<T>[];
  rows: T[];
  getRowKey: (row: T, index: number) => string;
  pagination?: TablePaginationState | null;
  onPageChange?: (page: number) => void;
  density?: 'default' | 'compact';
  className?: string;
};

const cellPadding = {
  default: 'px-4 py-3',
  compact: 'px-3 py-2',
} as const;

export function DataTable<T>({
  caption,
  columns,
  rows,
  getRowKey,
  pagination = null,
  onPageChange,
  density = 'default',
  className,
}: DataTableProps<T>) {
  const padding = cellPadding[density];
  const showPagination = pagination !== null && onPageChange !== undefined;

  return (
    <div className={cn('overflow-hidden rounded-lg border border-hairline bg-surface', className)}>
      <div className="overflow-x-auto">
        <table className="min-w-full text-sm">
          {caption ? <caption className="sr-only">{caption}</caption> : null}
          <thead className="border-b border-hairline bg-surface-muted">
            <tr>
              {columns.map((column) => (
                <th
                  key={column.id}
                  scope="col"
                  className={cn(
                    padding,
                    'font-medium text-foreground',
                    column.align === 'right' ? 'text-right' : 'text-left',
                    column.headerClassName,
                  )}
                >
                  {column.header}
                </th>
              ))}
            </tr>
          </thead>
          <tbody className="bg-surface">
            {rows.map((row, index) => (
              <tr
                key={getRowKey(row, index)}
                className={cn(
                  'border-b border-hairline last:border-0',
                  index % 2 === 0 ? 'bg-surface' : 'bg-surface-muted/60',
                )}
              >
                {columns.map((column) => (
                  <td
                    key={column.id}
                    className={cn(
                      padding,
                      'text-foreground',
                      column.align === 'right' ? 'text-right' : 'text-left',
                      column.cellClassName,
                    )}
                  >
                    {column.cell(row)}
                  </td>
                ))}
              </tr>
            ))}
          </tbody>
        </table>
      </div>

      {showPagination ? (
        <div
          className="flex flex-col gap-3 border-t border-hairline bg-surface px-4 py-3 sm:flex-row sm:items-center sm:justify-between"
          aria-label="Table pagination"
        >
          <p className="text-sm text-muted-foreground">
            {formatTablePaginationSummary(pagination)}
          </p>
          <div className="flex gap-2">
            <Button
              type="button"
              variant="secondary"
              disabled={pagination.page <= 1}
              onClick={() => {
                onPageChange(pagination.page - 1);
              }}
            >
              Previous
            </Button>
            <Button
              type="button"
              variant="secondary"
              disabled={pagination.page >= pagination.totalPages}
              onClick={() => {
                onPageChange(pagination.page + 1);
              }}
            >
              Next
            </Button>
          </div>
        </div>
      ) : null}
    </div>
  );
}

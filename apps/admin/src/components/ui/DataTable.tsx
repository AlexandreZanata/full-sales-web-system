import { useMemo, useState, type ReactNode } from 'react';

import { DataTableSearch } from '@/components/ui/DataTableSearch';
import { Button } from '@/components/ui/Button';
import { useDebouncedValue } from '@/lib/hooks/useDebouncedValue';
import { useI18n } from '@/lib/i18n/context';
import { formatPaginationSummary } from '@/lib/i18n/labels';
import { filterTableRows } from '@/lib/table/filterTableRows';
import type { TablePaginationState } from '@/lib/tablePagination';
import { cn } from '@/lib/utils';

const SEARCH_DEBOUNCE_MS = 300;

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
  onRowClick?: (row: T) => void;
  density?: 'default' | 'compact';
  className?: string;
  /** Client-side search over loaded rows. Default true. */
  searchable?: boolean;
  searchPlaceholder?: string;
  getSearchText?: (row: T) => string;
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
  onRowClick,
  density = 'default',
  className,
  searchable = true,
  searchPlaceholder,
  getSearchText,
}: DataTableProps<T>) {
  const { t } = useI18n();
  const padding = cellPadding[density];
  const showPagination = pagination !== null && onPageChange !== undefined;
  const [search, setSearch] = useState('');
  const debouncedSearch = useDebouncedValue(search, SEARCH_DEBOUNCE_MS);

  const visibleRows = useMemo(() => {
    if (!searchable) {
      return rows;
    }
    return filterTableRows(rows, debouncedSearch, getSearchText);
  }, [debouncedSearch, getSearchText, rows, searchable]);

  return (
    <div className={cn('overflow-hidden rounded-lg border border-hairline bg-surface', className)}>
      {searchable ? (
        <DataTableSearch
          value={search}
          placeholder={searchPlaceholder}
          onChange={setSearch}
        />
      ) : null}
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
            {visibleRows.length === 0 ? (
              <tr>
                <td
                  colSpan={columns.length}
                  className={cn(padding, 'text-center text-sm text-muted-foreground')}
                >
                  {rows.length === 0
                    ? t('common.table.empty')
                    : t('common.table.noSearchMatches')}
                </td>
              </tr>
            ) : (
              visibleRows.map((row, index) => (
                <tr
                  key={getRowKey(row, index)}
                  className={cn(
                    'border-b border-hairline last:border-0',
                    index % 2 === 0 ? 'bg-surface' : 'bg-surface-muted/60',
                    onRowClick && 'cursor-pointer hover:bg-surface-muted',
                  )}
                  onClick={
                    onRowClick
                      ? () => {
                          onRowClick(row);
                        }
                      : undefined
                  }
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
              ))
            )}
          </tbody>
        </table>
      </div>

      {showPagination ? (
        <div
          className="flex flex-col gap-3 border-t border-hairline bg-surface px-4 py-3 sm:flex-row sm:items-center sm:justify-between"
          aria-label={t('common.table.paginationAria')}
        >
          <p className="text-sm text-muted-foreground">{formatPaginationSummary(t, pagination)}</p>
          <div className="flex gap-2">
            <Button
              type="button"
              variant="secondary"
              disabled={pagination.page <= 1}
              onClick={() => {
                onPageChange(pagination.page - 1);
              }}
            >
              {t('common.previous')}
            </Button>
            <Button
              type="button"
              variant="secondary"
              disabled={pagination.page >= pagination.totalPages}
              onClick={() => {
                onPageChange(pagination.page + 1);
              }}
            >
              {t('common.next')}
            </Button>
          </div>
        </div>
      ) : null}
    </div>
  );
}

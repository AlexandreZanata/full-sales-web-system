import { Link } from '@tanstack/react-router';
import { useMemo } from 'react';

import { ActiveBadge } from '@/components/users/ActiveBadge';
import { DataTable, type DataTableColumn } from '@/components/ui/DataTable';
import type { ProductStockOverview } from '@/lib/api/types';
import { useI18n } from '@/lib/i18n/context';
import type { TablePaginationState } from '@/lib/tablePagination';

type StockOverviewTableProps = {
  rows: ProductStockOverview[];
  pagination: TablePaginationState | null;
  onPageChange: (page: number) => void;
};

export function StockOverviewTable({ rows, pagination, onPageChange }: StockOverviewTableProps) {
  const { t } = useI18n();

  const columns: DataTableColumn<ProductStockOverview>[] = useMemo(
    () => [
      {
        id: 'sku',
        header: t('forms.fields.sku'),
        cell: (row) => row.sku,
      },
      {
        id: 'name',
        header: t('common.table.name'),
        cell: (row) => (
          <Link
            to="/products/$id"
            params={{ id: row.productId }}
            className="font-medium hover:underline"
          >
            {row.name}
          </Link>
        ),
      },
      {
        id: 'available',
        header: t('inventory.overview.columns.available'),
        align: 'right',
        cell: (row) => (
          <span className="font-medium tabular-nums">
            {t('products.stock.units').replace('{count}', String(row.available))}
          </span>
        ),
      },
      {
        id: 'balanceTotal',
        header: t('inventory.overview.columns.balanceTotal'),
        align: 'right',
        cell: (row) => (
          <span className="tabular-nums text-muted-foreground">
            {t('products.stock.units').replace('{count}', String(row.balanceTotal))}
          </span>
        ),
      },
      {
        id: 'reserved',
        header: t('inventory.overview.columns.reserved'),
        align: 'right',
        cell: (row) => (
          <span className="tabular-nums text-muted-foreground">
            {t('products.stock.units').replace('{count}', String(row.reserved))}
          </span>
        ),
      },
      {
        id: 'active',
        header: t('forms.fields.status'),
        cell: (row) => <ActiveBadge active={row.active} />,
      },
    ],
    [t],
  );

  return (
    <DataTable
      caption={t('inventory.overview.caption')}
      columns={columns}
      rows={rows}
      getRowKey={(row) => row.productId}
      pagination={pagination}
      onPageChange={onPageChange}
    />
  );
}

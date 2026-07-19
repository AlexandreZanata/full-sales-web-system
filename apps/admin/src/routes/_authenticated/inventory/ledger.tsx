import { createFileRoute } from '@tanstack/react-router';
import { useQuery } from '@tanstack/react-query';
import { useMemo, useState } from 'react';

import { ProductSearchPicker } from '@/components/products/ProductSearchPicker';
import { DataTable, type DataTableColumn } from '@/components/ui/DataTable';
import { EmptyState } from '@/components/ui/EmptyState';
import { LoadingSpinner } from '@/components/ui/LoadingSpinner';
import { PageBackLink } from '@/components/ui/PageBackLink';
import { PageHeader } from '@/components/ui/PageHeader';
import { fetchMovements } from '@/lib/api/inventory';
import type { StockMovement } from '@/lib/api/types';
import { useI18n } from '@/lib/i18n/context';
import { cursorToTableState } from '@/lib/cursorPagination';

export const Route = createFileRoute('/_authenticated/inventory/ledger')({
  component: InventoryLedgerPage,
});

function InventoryLedgerPage() {
  const { t } = useI18n();
  const [productId, setProductId] = useState('');
  const [page, setPage] = useState(1);
  const [cursors, setCursors] = useState<(string | undefined)[]>([undefined]);
  const pageSize = 20;

  const movements = useQuery({
    queryKey: ['inventory', 'movements', productId, page, pageSize],
    queryFn: () =>
      fetchMovements({
        productId,
        limit: pageSize,
        cursor: cursors[page - 1],
      }),
    enabled: productId.length > 0,
  });

  const pagination = movements.data
    ? cursorToTableState(page, movements.data.pagination.has_more)
    : null;

  function handlePageChange(nextPage: number) {
    const nextCursor = movements.data?.pagination.next_cursor;
    if (nextPage > page && nextCursor) {
      setCursors((prev) => {
        const copy = [...prev];
        copy[page] = nextCursor;
        return copy;
      });
    }
    setPage(nextPage);
  }

  const columns: DataTableColumn<StockMovement>[] = useMemo(
    () => [
      {
        id: 'createdAt',
        header: t('common.table.date'),
        cell: (row) => new Date(row.createdAt).toLocaleString('pt-BR'),
      },
      {
        id: 'movementType',
        header: t('inventory.ledger.columns.type'),
        cell: (row) => row.movementType,
      },
      {
        id: 'quantity',
        header: t('inventory.ledger.columns.quantity'),
        cell: (row) => String(row.quantity),
      },
      {
        id: 'reason',
        header: t('inventory.ledger.columns.reason'),
        cell: (row) => row.reason ?? '—',
      },
    ],
    [t],
  );

  return (
    <div>
      <PageHeader
        title={t('inventory.ledger.title')}
        description={t('inventory.ledger.description')}
        back={<PageBackLink label={t('common.backTo.inventory')} to="/inventory" />}
      />

      <div className="mb-4 max-w-md">
        <ProductSearchPicker
          label={t('inventory.ledger.productFilter')}
          name="productId"
          value={productId}
          onChange={(nextProductId) => {
            setProductId(nextProductId);
            setPage(1);
            setCursors([undefined]);
          }}
        />
      </div>

      {!productId ? (
        <EmptyState
          title={t('inventory.ledger.selectProduct.title')}
          description={t('inventory.ledger.selectProduct.description')}
        />
      ) : movements.isLoading ? (
        <div className="flex justify-center py-16">
          <LoadingSpinner />
        </div>
      ) : movements.data && movements.data.data.length > 0 ? (
        <DataTable
          caption={t('inventory.ledger.caption')}
          columns={columns}
          rows={movements.data.data}
          getRowKey={(row) => row.id}
          pagination={pagination}
          onPageChange={handlePageChange}
        />
      ) : (
        <EmptyState
          title={t('inventory.ledger.empty.title')}
          description={t('inventory.ledger.empty.description')}
        />
      )}
    </div>
  );
}

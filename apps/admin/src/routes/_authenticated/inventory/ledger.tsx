import { createFileRoute } from '@tanstack/react-router';
import { useQuery } from '@tanstack/react-query';
import { useState } from 'react';

import { DataTable, type DataTableColumn } from '@/components/ui/DataTable';
import { EmptyState } from '@/components/ui/EmptyState';
import { LoadingSpinner } from '@/components/ui/LoadingSpinner';
import { PageBackLink } from '@/components/ui/PageBackLink';
import { PageHeader } from '@/components/ui/PageHeader';
import { Select } from '@/components/ui/Select';
import { fetchMovements } from '@/lib/api/inventory';
import { fetchProductsForPicker } from '@/lib/api/products';
import type { StockMovement } from '@/lib/api/types';
import { paginatedResponseToTable } from '@/lib/tablePagination';

export const Route = createFileRoute('/_authenticated/inventory/ledger')({
  component: InventoryLedgerPage,
});

const columns: DataTableColumn<StockMovement>[] = [
  {
    id: 'createdAt',
    header: 'Date',
    cell: (row) => new Date(row.createdAt).toLocaleString('pt-BR'),
  },
  {
    id: 'movementType',
    header: 'Type',
    cell: (row) => row.movementType,
  },
  {
    id: 'quantity',
    header: 'Quantity',
    cell: (row) => String(row.quantity),
  },
  {
    id: 'reason',
    header: 'Reason',
    cell: (row) => row.reason ?? '—',
  },
];

function InventoryLedgerPage() {
  const [productId, setProductId] = useState('');
  const [page, setPage] = useState(1);
  const pageSize = 20;

  const products = useQuery({
    queryKey: ['products', 'picker'],
    queryFn: fetchProductsForPicker,
  });

  const movements = useQuery({
    queryKey: ['inventory', 'movements', productId, page, pageSize],
    queryFn: () => fetchMovements({ productId, page, pageSize }),
    enabled: productId.length > 0,
  });

  const pagination = movements.data ? paginatedResponseToTable(movements.data) : null;

  return (
    <div>
      <PageHeader
        title="Inventory ledger"
        description="Append-only movement history for a product."
        back={<PageBackLink label="Back to inventory" to="/inventory" />}
      />

      <div className="mb-4 max-w-md">
        <Select
          label="Product"
          name="productId"
          value={productId}
          disabled={products.isLoading}
          onChange={(event) => {
            setProductId(event.target.value);
            setPage(1);
          }}
        >
          <option value="">Select product</option>
          {products.data?.map((product) => (
            <option key={product.id} value={product.id}>
              {product.sku} — {product.name}
            </option>
          ))}
        </Select>
      </div>

      {!productId ? (
        <EmptyState
          title="Select a product"
          description="Choose a product to load its movement ledger."
        />
      ) : movements.isLoading ? (
        <div className="flex justify-center py-16">
          <LoadingSpinner />
        </div>
      ) : movements.data && movements.data.items.length > 0 ? (
        <DataTable
          caption="Stock movements"
          columns={columns}
          rows={movements.data.items}
          getRowKey={(row) => row.id}
          pagination={pagination}
          onPageChange={setPage}
        />
      ) : (
        <EmptyState
          title="No movements found"
          description="Record an adjustment or confirm a sale to populate the ledger."
        />
      )}
    </div>
  );
}

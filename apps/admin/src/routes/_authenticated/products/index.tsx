import { Link, createFileRoute } from '@tanstack/react-router';
import { useQuery } from '@tanstack/react-query';
import { useState } from 'react';

import { ActiveBadge } from '@/components/users/ActiveBadge';
import { Button } from '@/components/ui/Button';
import { DataTable, type DataTableColumn } from '@/components/ui/DataTable';
import { EmptyState } from '@/components/ui/EmptyState';
import { LoadingSpinner } from '@/components/ui/LoadingSpinner';
import { PageHeader } from '@/components/ui/PageHeader';
import { fetchProducts } from '@/lib/api/products';
import type { ProductSummary } from '@/lib/api/types';
import { formatMoney } from '@/lib/products/formatPrice';
import { paginatedResponseToTable } from '@/lib/tablePagination';

export const Route = createFileRoute('/_authenticated/products/')({
  component: ProductsListPage,
});

const columns: DataTableColumn<ProductSummary>[] = [
  {
    id: 'sku',
    header: 'SKU',
    cell: (row) => row.sku,
  },
  {
    id: 'name',
    header: 'Name',
    cell: (row) => (
      <Link to="/products/$id" params={{ id: row.id }} className="font-medium hover:underline">
        {row.name}
      </Link>
    ),
  },
  {
    id: 'price',
    header: 'Price',
    cell: (row) => formatMoney(row.priceAmount, row.priceCurrency),
  },
  {
    id: 'active',
    header: 'Status',
    cell: (row) => <ActiveBadge active={row.active} />,
  },
];

function ProductsListPage() {
  const [page, setPage] = useState(1);
  const pageSize = 20;

  const products = useQuery({
    queryKey: ['products', page, pageSize],
    queryFn: () => fetchProducts({ page, pageSize }),
  });

  const pagination = products.data ? paginatedResponseToTable(products.data) : null;

  return (
    <div>
      <PageHeader
        title="Products"
        description="Manage the product catalog and pricing."
        actions={
          <Link to="/products/new">
            <Button>New product</Button>
          </Link>
        }
      />

      {products.isLoading ? (
        <div className="flex justify-center py-16">
          <LoadingSpinner />
        </div>
      ) : products.data && products.data.items.length > 0 ? (
        <DataTable
          caption="Products"
          columns={columns}
          rows={products.data.items}
          getRowKey={(row) => row.id}
          pagination={pagination}
          onPageChange={setPage}
        />
      ) : (
        <EmptyState
          title="No products found"
          description="Create the first product to get started."
        />
      )}
    </div>
  );
}

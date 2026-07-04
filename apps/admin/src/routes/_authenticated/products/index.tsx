import { Link, createFileRoute } from '@tanstack/react-router';
import { useQuery } from '@tanstack/react-query';
import { useMemo, useState } from 'react';

import { ActiveBadge } from '@/components/users/ActiveBadge';
import { Button } from '@/components/ui/Button';
import { DataTable, type DataTableColumn } from '@/components/ui/DataTable';
import { EmptyState } from '@/components/ui/EmptyState';
import { LoadingSpinner } from '@/components/ui/LoadingSpinner';
import { PageHeader } from '@/components/ui/PageHeader';
import { fetchProducts } from '@/lib/api/products';
import type { ProductSummary } from '@/lib/api/types';
import { useI18n } from '@/lib/i18n/context';
import { formatMoney } from '@/lib/products/formatPrice';
import { paginatedResponseToTable } from '@/lib/tablePagination';

export const Route = createFileRoute('/_authenticated/products/')({
  component: ProductsListPage,
});

function ProductsListPage() {
  const { t } = useI18n();
  const [page, setPage] = useState(1);
  const pageSize = 20;

  const products = useQuery({
    queryKey: ['products', page, pageSize],
    queryFn: () => fetchProducts({ page, pageSize }),
  });

  const pagination = products.data ? paginatedResponseToTable(products.data) : null;

  const columns: DataTableColumn<ProductSummary>[] = useMemo(
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
          <Link to="/products/$id" params={{ id: row.id }} className="font-medium hover:underline">
            {row.name}
          </Link>
        ),
      },
      {
        id: 'price',
        header: t('forms.fields.price'),
        cell: (row) => formatMoney(row.priceAmount, row.priceCurrency),
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
    <div>
      <PageHeader
        title={t('products.list.title')}
        description={t('products.list.description')}
        actions={
          <Link to="/products/new">
            <Button>{t('products.list.newProduct')}</Button>
          </Link>
        }
      />

      {products.isLoading ? (
        <div className="flex justify-center py-16">
          <LoadingSpinner />
        </div>
      ) : products.data && products.data.items.length > 0 ? (
        <DataTable
          caption={t('products.list.caption')}
          columns={columns}
          rows={products.data.items}
          getRowKey={(row) => row.id}
          pagination={pagination}
          onPageChange={setPage}
        />
      ) : (
        <EmptyState
          title={t('products.list.empty.title')}
          description={t('products.list.empty.description')}
        />
      )}
    </div>
  );
}

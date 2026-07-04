import { Link, createFileRoute } from '@tanstack/react-router';
import { useQuery } from '@tanstack/react-query';
import { useMemo, useState } from 'react';

import { ActiveBadge } from '@/components/users/ActiveBadge';
import { Button } from '@/components/ui/Button';
import { DataTable, type DataTableColumn } from '@/components/ui/DataTable';
import { EmptyState } from '@/components/ui/EmptyState';
import { Input } from '@/components/ui/Input';
import { LoadingSpinner } from '@/components/ui/LoadingSpinner';
import { PageHeader } from '@/components/ui/PageHeader';
import { Select } from '@/components/ui/Select';
import { fetchProducts } from '@/lib/api/products';
import type { ProductSummary } from '@/lib/api/types';
import { ACTIVE_FILTERS, type ActiveFilter } from '@/lib/commerces/constants';
import { useI18n } from '@/lib/i18n/context';
import { activeFilterLabel } from '@/lib/i18n/labels';
import { formatMoney } from '@/lib/products/formatPrice';
import { paginatedResponseToTable } from '@/lib/tablePagination';

export const Route = createFileRoute('/_authenticated/products/')({
  component: ProductsListPage,
});

function matchesSearch(product: ProductSummary, search: string): boolean {
  const normalized = search.trim().toLowerCase();
  if (!normalized) {
    return true;
  }
  return (
    product.name.toLowerCase().includes(normalized) ||
    product.sku.toLowerCase().includes(normalized)
  );
}

function ProductsListPage() {
  const { t } = useI18n();
  const [page, setPage] = useState(1);
  const [activeFilter, setActiveFilter] = useState<ActiveFilter>('');
  const [search, setSearch] = useState('');
  const pageSize = 20;

  const products = useQuery({
    queryKey: ['products', page, pageSize, activeFilter],
    queryFn: () => fetchProducts({ page, pageSize, active: activeFilter }),
  });

  const filteredItems = useMemo(
    () => (products.data?.items ?? []).filter((item) => matchesSearch(item, search)),
    [products.data?.items, search],
  );

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

  const hasFilters = Boolean(activeFilter || search.trim());

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

      <div className="mb-4 grid gap-4 sm:grid-cols-2">
        <Select
          label={t('products.list.filterByStatus')}
          value={activeFilter}
          onChange={(event) => {
            setActiveFilter(event.target.value as ActiveFilter);
            setPage(1);
          }}
        >
          {ACTIVE_FILTERS.map((value) => (
            <option key={value || 'all'} value={value}>
              {activeFilterLabel(t, value)}
            </option>
          ))}
        </Select>
        <Input
          label={t('products.list.searchPlaceholder')}
          name="search"
          value={search}
          onChange={(event) => {
            setSearch(event.target.value);
          }}
        />
      </div>

      {products.isLoading ? (
        <div className="flex justify-center py-16">
          <LoadingSpinner />
        </div>
      ) : filteredItems.length > 0 ? (
        <DataTable
          caption={t('products.list.caption')}
          columns={columns}
          rows={filteredItems}
          getRowKey={(row) => row.id}
          pagination={pagination}
          onPageChange={setPage}
        />
      ) : (
        <EmptyState
          title={t('products.list.empty.title')}
          description={
            hasFilters
              ? t('products.list.empty.descriptionFiltered')
              : t('products.list.empty.description')
          }
          action={
            hasFilters ? undefined : (
              <Link to="/products/new">
                <Button>{t('products.list.newProduct')}</Button>
              </Link>
            )
          }
        />
      )}
    </div>
  );
}

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
import { fetchCategoriesForPicker } from '@/lib/api/categories';
import { fetchProducts } from '@/lib/api/products';
import type { ProductSummary } from '@/lib/api/types';
import { ACTIVE_FILTERS, type ActiveFilter } from '@/lib/commerces/constants';
import { useI18n } from '@/lib/i18n/context';
import { activeFilterLabel } from '@/lib/i18n/labels';
import { formatMoney } from '@/lib/products/formatPrice';
import { cursorToTableState } from '@/lib/cursorPagination';

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

function matchesCategory(product: ProductSummary, categoryId: string): boolean {
  if (!categoryId) {
    return true;
  }
  return product.categoryId === categoryId;
}

function ProductsListPage() {
  const { t } = useI18n();
  const [page, setPage] = useState(1);
  const [cursors, setCursors] = useState<(string | undefined)[]>([undefined]);
  const [activeFilter, setActiveFilter] = useState<ActiveFilter>('');
  const [categoryFilter, setCategoryFilter] = useState('');
  const [search, setSearch] = useState('');
  const pageSize = 20;

  const categories = useQuery({
    queryKey: ['categories', 'picker'],
    queryFn: fetchCategoriesForPicker,
  });

  const products = useQuery({
    queryKey: ['products', page, pageSize, activeFilter],
    queryFn: () =>
      fetchProducts({
        limit: pageSize,
        cursor: cursors[page - 1],
        active: activeFilter,
      }),
  });

  const filteredItems = useMemo(
    () =>
      (products.data?.data ?? [])
        .filter((item) => matchesSearch(item, search))
        .filter((item) => matchesCategory(item, categoryFilter)),
    [products.data?.data, search, categoryFilter],
  );

  const pagination = products.data
    ? cursorToTableState(page, products.data.pagination.has_more)
    : null;

  function handlePageChange(nextPage: number) {
    if (nextPage > page && products.data?.pagination.next_cursor) {
      setCursors((prev) => {
        const copy = [...prev];
        copy[page] = products.data?.pagination.next_cursor ?? undefined;
        return copy;
      });
    }
    setPage(nextPage);
  }

  function resetPagination() {
    setPage(1);
    setCursors([undefined]);
  }

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
        id: 'category',
        header: t('forms.fields.category'),
        cell: (row) => row.categoryName ?? '—',
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
      {
        id: 'actions',
        header: t('common.table.actions'),
        align: 'right',
        cell: (row) => (
          <div className="flex flex-wrap justify-end gap-2">
            <Link to="/products/$id" params={{ id: row.id }}>
              <Button variant="secondary" className="h-8 min-h-8 px-3 text-xs">
                {t('common.edit')}
              </Button>
            </Link>
            <Link to="/products/$id" params={{ id: row.id }} search={{ tab: 'images' }}>
              <Button variant="secondary" className="h-8 min-h-8 px-3 text-xs">
                {t('products.actions.manageImages')}
              </Button>
            </Link>
          </div>
        ),
      },
    ],
    [t],
  );

  const hasFilters = Boolean(activeFilter || categoryFilter || search.trim());

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

      <div className="mb-4 grid gap-4 sm:grid-cols-3">
        <Select
          label={t('products.list.filterByStatus')}
          value={activeFilter}
          onChange={(event) => {
            setActiveFilter(event.target.value as ActiveFilter);
            resetPagination();
          }}
        >
          {ACTIVE_FILTERS.map((value) => (
            <option key={value || 'all'} value={value}>
              {activeFilterLabel(t, value)}
            </option>
          ))}
        </Select>
        <Select
          label={t('products.list.filterByCategory')}
          value={categoryFilter}
          onChange={(event) => {
            setCategoryFilter(event.target.value);
            setPage(1);
          }}
        >
          <option value="">{t('common.filter.allCategories')}</option>
          {(categories.data ?? []).map((category) => (
            <option key={category.id} value={category.id}>
              {category.name}
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
          onPageChange={handlePageChange}
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

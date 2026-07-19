import { Link, createFileRoute } from '@tanstack/react-router';
import { useQuery } from '@tanstack/react-query';
import { useMemo, useState } from 'react';

import { productListColumns } from '@/components/products/productListColumns';
import { Button } from '@/components/ui/Button';
import { DataTable } from '@/components/ui/DataTable';
import { EmptyState } from '@/components/ui/EmptyState';
import { LoadingSpinner } from '@/components/ui/LoadingSpinner';
import { PageHeader } from '@/components/ui/PageHeader';
import { Select } from '@/components/ui/Select';
import { fetchCategoriesForPicker } from '@/lib/api/categories';
import { fetchProducts } from '@/lib/api/products';
import type { ProductSummary } from '@/lib/api/types';
import { useCatalogRevision } from '@/lib/catalog/useCatalogRevision';
import { ACTIVE_FILTERS, type ActiveFilter } from '@/lib/commerces/constants';
import { useI18n } from '@/lib/i18n/context';
import { activeFilterLabel } from '@/lib/i18n/labels';
import { cursorToTableState } from '@/lib/cursorPagination';

export const Route = createFileRoute('/_authenticated/products/')({
  component: ProductsListPage,
});

function matchesCategory(product: ProductSummary, categoryId: string): boolean {
  if (!categoryId) {
    return true;
  }
  return product.categoryId === categoryId;
}

function ProductsListPage() {
  const { t } = useI18n();
  const catalogRevision = useCatalogRevision();
  const [page, setPage] = useState(1);
  const [cursors, setCursors] = useState<(string | undefined)[]>([undefined]);
  const [activeFilter, setActiveFilter] = useState<ActiveFilter>('');
  const [categoryFilter, setCategoryFilter] = useState('');
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
    () => (products.data?.data ?? []).filter((item) => matchesCategory(item, categoryFilter)),
    [products.data?.data, categoryFilter],
  );

  const pagination = products.data
    ? cursorToTableState(page, products.data.pagination.has_more)
    : null;

  function handlePageChange(nextPage: number) {
    const nextCursor = products.data?.pagination.next_cursor;
    if (nextPage > page && nextCursor) {
      setCursors((prev) => {
        const copy = [...prev];
        copy[page] = nextCursor;
        return copy;
      });
    }
    setPage(nextPage);
  }

  function resetPagination() {
    setPage(1);
    setCursors([undefined]);
  }

  const columns = useMemo(() => productListColumns(t, catalogRevision), [catalogRevision, t]);
  const hasFilters = Boolean(activeFilter || categoryFilter);

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

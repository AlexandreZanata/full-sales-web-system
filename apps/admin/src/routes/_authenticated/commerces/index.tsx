import { Link, createFileRoute } from '@tanstack/react-router';
import { useQuery } from '@tanstack/react-query';
import { useMemo, useState } from 'react';

import { commerceListColumns } from '@/components/commerces/commerceListColumns';
import { Button } from '@/components/ui/Button';
import { DataTable } from '@/components/ui/DataTable';
import { EmptyState } from '@/components/ui/EmptyState';
import { LoadingSpinner } from '@/components/ui/LoadingSpinner';
import { PageHeader } from '@/components/ui/PageHeader';
import { Select } from '@/components/ui/Select';
import { fetchCommerceRegistrations } from '@/lib/api/commerceRegistrations';
import { fetchCommerces } from '@/lib/api/commerces';
import { ACTIVE_FILTERS, type ActiveFilter } from '@/lib/commerces/constants';
import { cursorToTableState } from '@/lib/cursorPagination';
import { useI18n } from '@/lib/i18n/context';
import { activeFilterLabel } from '@/lib/i18n/labels';

export const Route = createFileRoute('/_authenticated/commerces/')({
  component: CommercesListPage,
});

function CommercesListPage() {
  const { t } = useI18n();
  const [page, setPage] = useState(1);
  const [cursors, setCursors] = useState<(string | undefined)[]>([undefined]);
  const [activeFilter, setActiveFilter] = useState<ActiveFilter>('');
  const pageSize = 20;

  const commerces = useQuery({
    queryKey: ['commerces', page, pageSize, activeFilter],
    queryFn: () =>
      fetchCommerces({
        limit: pageSize,
        cursor: cursors[page - 1],
        active: activeFilter,
      }),
  });

  const pendingReviews = useQuery({
    queryKey: ['commerce-registrations', 'pending'],
    queryFn: () => fetchCommerceRegistrations({ limit: 100, status: 'PendingReview' }),
  });

  const pendingIds = useMemo(
    () => new Set((pendingReviews.data?.data ?? []).map((row) => row.id)),
    [pendingReviews.data?.data],
  );

  const pagination = commerces.data
    ? cursorToTableState(page, commerces.data.pagination.has_more)
    : null;

  function handlePageChange(nextPage: number) {
    const nextCursor = commerces.data?.pagination.next_cursor;
    if (nextPage > page && nextCursor) {
      setCursors((prev) => {
        const copy = [...prev];
        copy[page] = nextCursor;
        return copy;
      });
    }
    setPage(nextPage);
  }

  const columns = useMemo(() => commerceListColumns(t, pendingIds), [pendingIds, t]);
  const items = commerces.data?.data ?? [];

  return (
    <div>
      <PageHeader
        title={t('commerces.list.title')}
        description={t('commerces.list.description')}
        actions={
          <div className="flex flex-wrap gap-2">
            <Link to="/commerces/leads">
              <Button variant="secondary">{t('commerces.leads.openLeads')}</Button>
            </Link>
            <Link to="/commerces/new">
              <Button>{t('commerces.list.register')}</Button>
            </Link>
          </div>
        }
      />

      <div className="mb-4 max-w-xs">
        <Select
          label={t('commerces.list.filterByStatus')}
          value={activeFilter}
          onChange={(event) => {
            setActiveFilter(event.target.value as ActiveFilter);
            setPage(1);
            setCursors([undefined]);
          }}
        >
          {ACTIVE_FILTERS.map((value) => (
            <option key={value || 'all'} value={value}>
              {activeFilterLabel(t, value)}
            </option>
          ))}
        </Select>
      </div>

      {commerces.isLoading ? (
        <div className="flex justify-center py-16">
          <LoadingSpinner />
        </div>
      ) : items.length > 0 ? (
        <DataTable
          caption={t('commerces.list.caption')}
          columns={columns}
          rows={items}
          getRowKey={(row) => row.id}
          pagination={pagination}
          onPageChange={handlePageChange}
        />
      ) : (
        <EmptyState
          title={t('commerces.list.empty.title')}
          description={
            activeFilter
              ? t('commerces.list.empty.descriptionFiltered')
              : t('commerces.list.empty.descriptionDefault')
          }
        />
      )}
    </div>
  );
}

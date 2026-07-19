import { Link, createFileRoute } from '@tanstack/react-router';
import { useQuery } from '@tanstack/react-query';
import { useMemo, useState } from 'react';

import { Button } from '@/components/ui/Button';
import { DataTable, type DataTableColumn } from '@/components/ui/DataTable';
import { EmptyState } from '@/components/ui/EmptyState';
import { LoadingSpinner } from '@/components/ui/LoadingSpinner';
import { PageHeader } from '@/components/ui/PageHeader';
import { Select } from '@/components/ui/Select';
import { TableActions, tableActionClass } from '@/components/ui/TableActions';
import { fetchCommerceRegistrations } from '@/lib/api/commerceRegistrations';
import type { CommerceRegistration, RegistrationStatus } from '@/lib/api/types';
import { formatCnpj } from '@/lib/commerces/cnpj';
import { cursorToTableState } from '@/lib/cursorPagination';
import { useI18n } from '@/lib/i18n/context';

const STATUS_FILTERS: Array<RegistrationStatus | ''> = ['PendingReview', 'Active', 'Rejected', ''];

export const Route = createFileRoute('/_authenticated/commerces/registrations/')({
  component: CommerceRegistrationsPage,
});

function CommerceRegistrationsPage() {
  const { t } = useI18n();
  const [page, setPage] = useState(1);
  const [cursors, setCursors] = useState<(string | undefined)[]>([undefined]);
  const [statusFilter, setStatusFilter] = useState<RegistrationStatus | ''>('PendingReview');
  const pageSize = 20;

  const registrations = useQuery({
    queryKey: ['commerce-registrations', page, pageSize, statusFilter],
    queryFn: () =>
      fetchCommerceRegistrations({
        limit: pageSize,
        cursor: cursors[page - 1],
        status: statusFilter || undefined,
      }),
  });

  const pagination = registrations.data
    ? cursorToTableState(page, registrations.data.pagination.has_more)
    : null;

  const columns = useMemo<DataTableColumn<CommerceRegistration>[]>(
    () => [
      {
        id: 'tradeName',
        header: t('commerces.registrations.columns.tradeName'),
        cell: (row) => row.tradeName,
      },
      {
        id: 'cnpj',
        header: t('commerces.registrations.columns.cnpj'),
        cell: (row) => formatCnpj(row.cnpj),
      },
      {
        id: 'status',
        header: t('commerces.registrations.columns.status'),
        cell: (row) => t(`commerces.registrations.status.${row.registrationStatus}`),
      },
      {
        id: 'actions',
        header: t('common.table.actions'),
        align: 'right',
        cell: (row) => (
          <TableActions>
            <Link
              className={tableActionClass('warning')}
              to="/commerces/registrations/$id"
              params={{ id: row.id }}
            >
              {t('common.review')}
            </Link>
          </TableActions>
        ),
      },
    ],
    [t],
  );

  function handlePageChange(nextPage: number) {
    const nextCursor = registrations.data?.pagination.next_cursor;
    if (nextPage > page && nextCursor) {
      setCursors((prev) => {
        const copy = [...prev];
        copy[page] = nextCursor;
        return copy;
      });
    }
    setPage(nextPage);
  }

  return (
    <div className="space-y-6">
      <PageHeader
        title={t('commerces.registrations.title')}
        description={t('commerces.registrations.description')}
        back={<Link to="/commerces">{t('common.backTo.commerces')}</Link>}
      />

      <div className="flex flex-wrap items-end gap-4">
        <Select
          label={t('commerces.registrations.filterStatus')}
          value={statusFilter}
          onChange={(event) => {
            setStatusFilter(event.target.value as RegistrationStatus | '');
            setPage(1);
            setCursors([undefined]);
          }}
        >
          {STATUS_FILTERS.map((value) => (
            <option key={value || 'all'} value={value}>
              {value ? t(`commerces.registrations.status.${value}`) : t('common.all')}
            </option>
          ))}
        </Select>
        <Link to="/commerces/new">
          <Button variant="secondary">{t('commerces.list.register')}</Button>
        </Link>
      </div>

      {registrations.isLoading ? (
        <div className="flex justify-center py-16">
          <LoadingSpinner />
        </div>
      ) : registrations.data && registrations.data.data.length > 0 ? (
        <DataTable
          columns={columns}
          rows={registrations.data.data}
          getRowKey={(row) => row.id}
          pagination={pagination}
          onPageChange={handlePageChange}
        />
      ) : (
        <EmptyState
          title={t('commerces.registrations.empty.title')}
          description={t('commerces.registrations.empty.description')}
        />
      )}
    </div>
  );
}

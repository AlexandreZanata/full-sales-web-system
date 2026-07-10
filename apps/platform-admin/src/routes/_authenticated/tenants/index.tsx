import { Link, createFileRoute } from '@tanstack/react-router';
import { useQuery } from '@tanstack/react-query';
import { useState } from 'react';

import { Button } from '@/components/ui/Button';
import { DataTable, type DataTableColumn } from '@/components/ui/DataTable';
import { EmptyState } from '@/components/ui/EmptyState';
import { LoadingSpinner } from '@/components/ui/LoadingSpinner';
import { PageHeader } from '@/components/ui/PageHeader';
import { Select } from '@/components/ui/Select';
import { fetchTenants } from '@/lib/api/tenants';
import type { TenantListItem } from '@/lib/api/types';
import { cursorToTableState } from '@/lib/cursorPagination';
import { formatDateTime } from '@/lib/formatDateTime';
import { useI18n } from '@/lib/i18n/context';
import { tenantStatusTone } from '@/lib/platform-tokens';

export const Route = createFileRoute('/_authenticated/tenants/')({
  component: TenantsPage,
});

function TenantsPage() {
  const { t } = useI18n();
  const [page, setPage] = useState(1);
  const [cursors, setCursors] = useState<(string | undefined)[]>([undefined]);
  const [status, setStatus] = useState('');
  const pageSize = 20;

  const tenants = useQuery({
    queryKey: ['tenants', page, status],
    queryFn: () =>
      fetchTenants({ limit: pageSize, cursor: cursors[page - 1], status: status || undefined }),
  });

  const columns: DataTableColumn<TenantListItem>[] = [
    { id: 'name', header: t('common.name'), cell: (row) => row.displayName },
    {
      id: 'status',
      header: t('common.status'),
      cell: (row) => <span className={tenantStatusTone(row.status)}>{row.status}</span>,
    },
    { id: 'created', header: t('common.createdAt'), cell: (row) => formatDateTime(row.createdAt) },
    {
      id: 'actions',
      header: t('common.actions'),
      cell: (row) => (
        <Link
          to="/tenants/$id"
          params={{ id: row.id }}
          className="text-sm underline-offset-2 hover:underline"
        >
          View
        </Link>
      ),
    },
  ];

  const pagination = tenants.data
    ? cursorToTableState(page, tenants.data.pagination.has_more)
    : null;

  return (
    <div className="space-y-4">
      <PageHeader
        title={t('tenants.title')}
        actions={
          <Link to="/tenants/new">
            <Button>{t('tenants.new')}</Button>
          </Link>
        }
      />
      <Select
        label={t('common.status')}
        value={status}
        onChange={(e) => {
          setStatus(e.target.value);
          setPage(1);
          setCursors([undefined]);
        }}
      >
        <option value="">{t('common.all')}</option>
        <option value="Active">Active</option>
        <option value="Trial">Trial</option>
        <option value="PastDue">PastDue</option>
        <option value="Suspended">Suspended</option>
      </Select>
      {tenants.isLoading ? <LoadingSpinner /> : null}
      {tenants.data?.data.length ? (
        <DataTable
          columns={columns}
          rows={tenants.data.data}
          getRowKey={(row) => row.id}
          pagination={pagination}
          onPageChange={(next) => {
            if (next > page && tenants.data.pagination.next_cursor) {
              setCursors((prev) => [...prev, tenants.data.pagination.next_cursor ?? undefined]);
            }
            setPage(next);
          }}
        />
      ) : tenants.isSuccess ? (
        <EmptyState title={t('common.noResults')} />
      ) : null}
    </div>
  );
}

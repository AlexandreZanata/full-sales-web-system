import { createFileRoute, useNavigate } from '@tanstack/react-router';
import { useQuery } from '@tanstack/react-query';
import { useEffect, useState } from 'react';

import { CreateTenantDialog } from '@/components/tenants/CreateTenantDialog';
import { TenantDetailDialog } from '@/components/tenants/TenantDetailDialog';
import { TenantEditDialog } from '@/components/tenants/TenantEditDialog';
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

type TenantsSearch = {
  modal?: 'create' | 'edit' | 'view';
  id?: string;
};

export const Route = createFileRoute('/_authenticated/tenants/')({
  validateSearch: (search: Record<string, unknown>): TenantsSearch => {
    const modal =
      search.modal === 'create' || search.modal === 'edit' || search.modal === 'view'
        ? search.modal
        : undefined;
    const id = typeof search.id === 'string' && search.id.length > 0 ? search.id : undefined;
    return { modal, id };
  },
  component: TenantsPage,
});

function TenantsPage() {
  const { t } = useI18n();
  const navigate = useNavigate({ from: Route.fullPath });
  const search = Route.useSearch();
  const [page, setPage] = useState(1);
  const [cursors, setCursors] = useState<(string | undefined)[]>([undefined]);
  const [status, setStatus] = useState('');
  const [createOpen, setCreateOpen] = useState(false);
  const [viewId, setViewId] = useState<string | null>(null);
  const [editId, setEditId] = useState<string | null>(null);
  const pageSize = 20;

  useEffect(() => {
    if (search.modal === 'create') {
      setCreateOpen(true);
      setViewId(null);
      setEditId(null);
      return;
    }
    if (search.modal === 'edit' && search.id) {
      setEditId(search.id);
      setCreateOpen(false);
      setViewId(null);
      return;
    }
    if (search.modal === 'view' && search.id) {
      setViewId(search.id);
      setCreateOpen(false);
      setEditId(null);
    }
  }, [search.modal, search.id]);

  function clearModalSearch() {
    void navigate({ search: {}, replace: true });
  }

  function openCreate() {
    setCreateOpen(true);
    void navigate({ search: { modal: 'create' }, replace: true });
  }

  function openView(id: string) {
    setViewId(id);
    void navigate({ search: { modal: 'view', id }, replace: true });
  }

  function openEdit(id: string) {
    setEditId(id);
    void navigate({ search: { modal: 'edit', id }, replace: true });
  }

  function closeCreate() {
    setCreateOpen(false);
    clearModalSearch();
  }

  function closeView() {
    setViewId(null);
    clearModalSearch();
  }

  function closeEdit() {
    setEditId(null);
    clearModalSearch();
  }

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
        <span className="flex flex-wrap gap-3">
          <button
            type="button"
            className="text-sm underline-offset-2 hover:underline"
            onClick={() => {
              openView(row.id);
            }}
          >
            {t('tenants.view')}
          </button>
          <button
            type="button"
            className="text-sm underline-offset-2 hover:underline"
            onClick={() => {
              openEdit(row.id);
            }}
          >
            {t('tenants.edit')}
          </button>
        </span>
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
        actions={<Button onClick={openCreate}>{t('tenants.new')}</Button>}
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
      <CreateTenantDialog open={createOpen} onClose={closeCreate} />
      <TenantDetailDialog
        tenantId={viewId}
        onClose={closeView}
        onEdit={(id) => {
          closeView();
          openEdit(id);
        }}
      />
      <TenantEditDialog tenantId={editId} onClose={closeEdit} />
    </div>
  );
}

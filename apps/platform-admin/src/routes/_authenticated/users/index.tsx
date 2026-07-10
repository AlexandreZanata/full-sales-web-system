import { Link, createFileRoute } from '@tanstack/react-router';
import { useQuery } from '@tanstack/react-query';
import { useState } from 'react';

import { DataTable, type DataTableColumn } from '@/components/ui/DataTable';
import { EmptyState } from '@/components/ui/EmptyState';
import { Input } from '@/components/ui/Input';
import { LoadingSpinner } from '@/components/ui/LoadingSpinner';
import { PageHeader } from '@/components/ui/PageHeader';
import { fetchPlatformUsers } from '@/lib/api/users';
import type { PlatformUser } from '@/lib/api/types';
import { cursorToTableState } from '@/lib/cursorPagination';
import { useI18n } from '@/lib/i18n/context';

export const Route = createFileRoute('/_authenticated/users/')({
  component: UsersPage,
});

function UsersPage() {
  const { t } = useI18n();
  const [page, setPage] = useState(1);
  const [cursors, setCursors] = useState<(string | undefined)[]>([undefined]);
  const [emailPrefix, setEmailPrefix] = useState('');
  const pageSize = 20;

  const users = useQuery({
    queryKey: ['platform-users', page, emailPrefix],
    queryFn: () =>
      fetchPlatformUsers({
        limit: pageSize,
        cursor: cursors[page - 1],
        emailPrefix: emailPrefix || undefined,
      }),
  });

  const columns: DataTableColumn<PlatformUser>[] = [
    { id: 'name', header: t('common.name'), cell: (row) => row.name },
    { id: 'email', header: t('common.email'), cell: (row) => row.email },
    { id: 'tenant', header: 'Tenant', cell: (row) => row.tenant.displayName },
    { id: 'role', header: 'Role', cell: (row) => row.role },
    {
      id: 'actions',
      header: t('common.actions'),
      cell: (row) => (
        <Link
          to="/users/$id"
          params={{ id: row.id }}
          className="text-sm underline-offset-2 hover:underline"
        >
          View
        </Link>
      ),
    },
  ];

  const pagination = users.data ? cursorToTableState(page, users.data.pagination.has_more) : null;

  return (
    <div className="space-y-4">
      <PageHeader title={t('users.title')} />
      <Input
        label={t('common.search')}
        value={emailPrefix}
        onChange={(e) => {
          setEmailPrefix(e.target.value);
          setPage(1);
          setCursors([undefined]);
        }}
      />
      {users.isLoading ? <LoadingSpinner /> : null}
      {users.data?.data.length ? (
        <DataTable
          columns={columns}
          rows={users.data.data}
          getRowKey={(row) => row.id}
          pagination={pagination}
          onPageChange={(next) => {
            if (next > page && users.data.pagination.next_cursor) {
              setCursors((prev) => [...prev, users.data.pagination.next_cursor ?? undefined]);
            }
            setPage(next);
          }}
        />
      ) : users.isSuccess ? (
        <EmptyState title={t('common.noResults')} />
      ) : null}
    </div>
  );
}

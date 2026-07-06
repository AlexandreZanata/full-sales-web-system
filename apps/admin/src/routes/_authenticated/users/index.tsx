import { Link, createFileRoute } from '@tanstack/react-router';
import { useQuery } from '@tanstack/react-query';
import { useMemo, useState } from 'react';

import { ActiveBadge } from '@/components/users/ActiveBadge';
import { Button } from '@/components/ui/Button';
import { DataTable, type DataTableColumn } from '@/components/ui/DataTable';
import { EmptyState } from '@/components/ui/EmptyState';
import { LoadingSpinner } from '@/components/ui/LoadingSpinner';
import { PageHeader } from '@/components/ui/PageHeader';
import { Select } from '@/components/ui/Select';
import { fetchUsers } from '@/lib/api/users';
import type { User, UserRole } from '@/lib/api/types';
import { cursorToTableState } from '@/lib/cursorPagination';
import { useI18n } from '@/lib/i18n/context';
import { translateRole } from '@/lib/i18n/labels';
import { USER_ROLES } from '@/lib/users/constants';

export const Route = createFileRoute('/_authenticated/users/')({
  component: UsersListPage,
});

function UsersListPage() {
  const { t } = useI18n();
  const [page, setPage] = useState(1);
  const [cursors, setCursors] = useState<(string | undefined)[]>([undefined]);
  const [roleFilter, setRoleFilter] = useState<UserRole | ''>('');
  const pageSize = 20;

  const users = useQuery({
    queryKey: ['users', page, pageSize, roleFilter],
    queryFn: () =>
      fetchUsers({
        limit: pageSize,
        cursor: cursors[page - 1],
        role: roleFilter,
      }),
  });

  const pagination = users.data ? cursorToTableState(page, users.data.pagination.has_more) : null;

  function handlePageChange(nextPage: number) {
    const nextCursor = users.data?.pagination.next_cursor;
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

  const columns: DataTableColumn<User>[] = useMemo(
    () => [
      {
        id: 'name',
        header: t('common.table.name'),
        cell: (row) => (
          <Link to="/users/$id" params={{ id: row.id }} className="font-medium hover:underline">
            {row.name}
          </Link>
        ),
      },
      {
        id: 'email',
        header: t('common.table.email'),
        cell: (row) => row.email,
      },
      {
        id: 'role',
        header: t('common.table.role'),
        cell: (row) => translateRole(t, row.role),
      },
      {
        id: 'active',
        header: t('forms.fields.status'),
        cell: (row) => <ActiveBadge active={row.active} />,
      },
    ],
    [t],
  );

  const items = users.data?.data ?? [];

  return (
    <div>
      <PageHeader
        title={t('users.list.title')}
        description={t('users.list.description')}
        actions={
          <Link to="/users/new">
            <Button>{t('users.list.newUser')}</Button>
          </Link>
        }
      />

      <div className="mb-4 max-w-xs">
        <Select
          label={t('users.list.filterByRole')}
          value={roleFilter}
          onChange={(event) => {
            setRoleFilter(event.target.value as UserRole | '');
            resetPagination();
          }}
        >
          <option value="">{t('common.filter.allRoles')}</option>
          {USER_ROLES.map((role) => (
            <option key={role} value={role}>
              {translateRole(t, role)}
            </option>
          ))}
        </Select>
      </div>

      {users.isLoading ? (
        <div className="flex justify-center py-16">
          <LoadingSpinner />
        </div>
      ) : items.length > 0 ? (
        <DataTable
          caption={t('users.list.caption')}
          columns={columns}
          rows={items}
          getRowKey={(row) => row.id}
          pagination={pagination}
          onPageChange={handlePageChange}
        />
      ) : (
        <EmptyState
          title={t('users.list.empty.title')}
          description={
            roleFilter
              ? t('users.list.empty.descriptionFiltered')
              : t('users.list.empty.descriptionDefault')
          }
        />
      )}
    </div>
  );
}

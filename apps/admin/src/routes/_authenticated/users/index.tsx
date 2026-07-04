import { Link, createFileRoute } from '@tanstack/react-router';
import { useQuery } from '@tanstack/react-query';
import { useState } from 'react';

import { ActiveBadge } from '@/components/users/ActiveBadge';
import { Button } from '@/components/ui/Button';
import { DataTable, type DataTableColumn } from '@/components/ui/DataTable';
import { EmptyState } from '@/components/ui/EmptyState';
import { LoadingSpinner } from '@/components/ui/LoadingSpinner';
import { PageHeader } from '@/components/ui/PageHeader';
import { Select } from '@/components/ui/Select';
import { fetchUsers } from '@/lib/api/users';
import type { User, UserRole } from '@/lib/api/types';
import { USER_ROLE_LABELS, USER_ROLES } from '@/lib/users/constants';
import { paginatedResponseToTable } from '@/lib/tablePagination';

export const Route = createFileRoute('/_authenticated/users/')({
  component: UsersListPage,
});

const columns: DataTableColumn<User>[] = [
  {
    id: 'name',
    header: 'Name',
    cell: (row) => (
      <Link to="/users/$id" params={{ id: row.id }} className="font-medium hover:underline">
        {row.name}
      </Link>
    ),
  },
  {
    id: 'email',
    header: 'Email',
    cell: (row) => row.email,
  },
  {
    id: 'role',
    header: 'Role',
    cell: (row) => USER_ROLE_LABELS[row.role],
  },
  {
    id: 'active',
    header: 'Status',
    cell: (row) => <ActiveBadge active={row.active} />,
  },
];

function UsersListPage() {
  const [page, setPage] = useState(1);
  const [roleFilter, setRoleFilter] = useState<UserRole | ''>('');
  const pageSize = 20;

  const users = useQuery({
    queryKey: ['users', page, pageSize, roleFilter],
    queryFn: () => fetchUsers({ page, pageSize, role: roleFilter }),
  });

  const pagination = users.data ? paginatedResponseToTable(users.data) : null;

  return (
    <div>
      <PageHeader
        title="Users"
        description="Manage admin, driver, seller, and commerce contact accounts."
        actions={
          <Link to="/users/new">
            <Button>New user</Button>
          </Link>
        }
      />

      <div className="mb-4 max-w-xs">
        <Select
          label="Filter by role"
          value={roleFilter}
          onChange={(event) => {
            setRoleFilter(event.target.value as UserRole | '');
            setPage(1);
          }}
        >
          <option value="">All roles</option>
          {USER_ROLES.map((role) => (
            <option key={role} value={role}>
              {USER_ROLE_LABELS[role]}
            </option>
          ))}
        </Select>
      </div>

      {users.isLoading ? (
        <div className="flex justify-center py-16">
          <LoadingSpinner />
        </div>
      ) : users.data && users.data.items.length > 0 ? (
        <DataTable
          caption="Users"
          columns={columns}
          rows={users.data.items}
          getRowKey={(row) => row.id}
          pagination={pagination}
          onPageChange={setPage}
        />
      ) : (
        <EmptyState
          title="No users found"
          description={
            roleFilter
              ? 'Try another role filter or create a new user.'
              : 'Create the first user to get started.'
          }
        />
      )}
    </div>
  );
}

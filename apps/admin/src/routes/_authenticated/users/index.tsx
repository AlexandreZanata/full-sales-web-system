import { Link, createFileRoute } from '@tanstack/react-router';
import { useQuery, useQueryClient } from '@tanstack/react-query';
import { useMemo, useState } from 'react';

import { userListColumns } from '@/components/users/userListColumns';
import { Button } from '@/components/ui/Button';
import { ConfirmDialog } from '@/components/ui/ConfirmDialog';
import { DataTable } from '@/components/ui/DataTable';
import { EmptyState } from '@/components/ui/EmptyState';
import { LoadingSpinner } from '@/components/ui/LoadingSpinner';
import { PageHeader } from '@/components/ui/PageHeader';
import { Select } from '@/components/ui/Select';
import { useToast } from '@/hooks/useToast';
import { deactivateUser, fetchUsers, reactivateUser } from '@/lib/api/users';
import type { User, UserRole } from '@/lib/api/types';
import { cursorToTableState } from '@/lib/cursorPagination';
import { useI18n } from '@/lib/i18n/context';
import { translateRole } from '@/lib/i18n/labels';
import { USER_ROLES } from '@/lib/users/constants';

export const Route = createFileRoute('/_authenticated/users/')({
  component: UsersListPage,
});

type PendingAction = { user: User; kind: 'deactivate' | 'reactivate' };

function UsersListPage() {
  const { t } = useI18n();
  const toast = useToast();
  const queryClient = useQueryClient();
  const [page, setPage] = useState(1);
  const [cursors, setCursors] = useState<(string | undefined)[]>([undefined]);
  const [roleFilter, setRoleFilter] = useState<UserRole | ''>('');
  const [pending, setPending] = useState<PendingAction | null>(null);
  const [busy, setBusy] = useState(false);
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

  async function handleConfirm() {
    if (!pending) {
      return;
    }
    setBusy(true);
    try {
      if (pending.kind === 'deactivate') {
        await deactivateUser(pending.user.id);
        toast.success(t('users.toast.deactivated'));
      } else {
        await reactivateUser(pending.user.id);
        toast.success(t('users.toast.reactivated'));
      }
      await queryClient.invalidateQueries({ queryKey: ['users'] });
      setPending(null);
    } catch {
      toast.error(t('errors.actionFailed'));
    } finally {
      setBusy(false);
    }
  }

  const columns = useMemo(
    () =>
      userListColumns(
        t,
        (user) => {
          setPending({ user, kind: 'deactivate' });
        },
        (user) => {
          setPending({ user, kind: 'reactivate' });
        },
      ),
    [t],
  );
  const items = users.data?.data ?? [];
  const isDeactivate = pending?.kind === 'deactivate';

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
            setPage(1);
            setCursors([undefined]);
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

      <ConfirmDialog
        open={pending !== null}
        title={
          isDeactivate
            ? t('users.detail.deactivateDialog.title')
            : t('users.detail.reactivateDialog.title')
        }
        message={
          pending
            ? `${
                isDeactivate
                  ? t('users.detail.deactivateDialog.message')
                  : t('users.detail.reactivateDialog.message')
              } (${pending.user.name})`
            : ''
        }
        confirmLabel={
          isDeactivate
            ? t('users.detail.deactivateDialog.confirm')
            : t('users.detail.reactivateDialog.confirm')
        }
        destructive={isDeactivate}
        isLoading={busy}
        onCancel={() => {
          setPending(null);
        }}
        onConfirm={() => void handleConfirm()}
      />
    </div>
  );
}

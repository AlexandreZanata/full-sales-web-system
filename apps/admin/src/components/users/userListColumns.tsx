import { Link } from '@tanstack/react-router';

import { ActiveBadge } from '@/components/users/ActiveBadge';
import type { DataTableColumn } from '@/components/ui/DataTable';
import { TableActionButton, TableActions, tableActionClass } from '@/components/ui/TableActions';
import type { User } from '@/lib/api/types';
import { translateRole } from '@/lib/i18n/labels';
import type { MessageKey } from '@/lib/i18n/messages';

type Translate = (key: MessageKey) => string;

export function userListColumns(
  t: Translate,
  onDeactivate: (user: User) => void,
  onReactivate: (user: User) => void,
): DataTableColumn<User>[] {
  return [
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
    {
      id: 'actions',
      header: t('common.table.actions'),
      align: 'right',
      cell: (row) => (
        <TableActions>
          <Link to="/users/$id" params={{ id: row.id }} className={tableActionClass('open')}>
            {t('common.open')}
          </Link>
          {row.active ? (
            <TableActionButton tone="danger" onClick={() => onDeactivate(row)}>
              {t('common.deactivate')}
            </TableActionButton>
          ) : (
            <TableActionButton tone="success" onClick={() => onReactivate(row)}>
              {t('common.reactivate')}
            </TableActionButton>
          )}
        </TableActions>
      ),
    },
  ];
}

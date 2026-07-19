import { Link } from '@tanstack/react-router';
import { useMutation, useQueryClient } from '@tanstack/react-query';

import { TableActionButton, TableActions, tableActionClass } from '@/components/ui/TableActions';
import { ActiveBadge } from '@/components/users/ActiveBadge';
import type { DataTableColumn } from '@/components/ui/DataTable';
import { useToast } from '@/hooks/useToast';
import { activateCommerce } from '@/lib/api/commerces';
import type { CommerceSummary } from '@/lib/api/types';
import { formatCnpj } from '@/lib/commerces/cnpj';
import type { MessageKey } from '@/lib/i18n/messages';

type Translate = (key: MessageKey) => string;

export function commerceListColumns(
  t: Translate,
  pendingIds: Set<string>,
): DataTableColumn<CommerceSummary>[] {
  return [
    {
      id: 'cnpj',
      header: t('forms.fields.cnpj'),
      cell: (row) => formatCnpj(row.cnpj),
    },
    {
      id: 'tradeName',
      header: t('forms.fields.tradeName'),
      cell: (row) => (
        <Link to="/commerces/$id" params={{ id: row.id }} className="font-medium hover:underline">
          {row.tradeName || row.legalName}
        </Link>
      ),
    },
    {
      id: 'legalName',
      header: t('forms.fields.legalName'),
      cell: (row) => row.legalName,
    },
    {
      id: 'active',
      header: t('forms.fields.status'),
      cell: (row) =>
        pendingIds.has(row.id) ? (
          <span className="inline-flex rounded-full border border-amber-500/40 px-2.5 py-0.5 text-xs font-medium text-amber-700">
            {t('commerces.registrations.status.PendingReview')}
          </span>
        ) : (
          <ActiveBadge active={row.active} />
        ),
    },
    {
      id: 'actions',
      header: t('common.table.actions'),
      align: 'right',
      cell: (row) => <CommerceRowActions t={t} row={row} pending={pendingIds.has(row.id)} />,
    },
  ];
}

function CommerceRowActions({
  t,
  row,
  pending,
}: {
  t: Translate;
  row: CommerceSummary;
  pending: boolean;
}) {
  const toast = useToast();
  const queryClient = useQueryClient();

  const activate = useMutation({
    mutationFn: () => activateCommerce(row.id),
    onSuccess: async () => {
      await queryClient.invalidateQueries({ queryKey: ['commerces'] });
      toast.success(t('commerces.toast.reactivated'));
    },
    onError: () => toast.error(t('errors.actionFailed')),
  });

  return (
    <TableActions>
      {pending ? (
        <Link
          to="/commerces/registrations/$id"
          params={{ id: row.id }}
          className={tableActionClass('warning')}
          onClick={(event) => event.stopPropagation()}
        >
          {t('common.review')}
        </Link>
      ) : (
        <Link
          to="/commerces/$id"
          params={{ id: row.id }}
          className={tableActionClass('open')}
          onClick={(event) => event.stopPropagation()}
        >
          {t('common.open')}
        </Link>
      )}
      {!row.active && !pending ? (
        <TableActionButton
          tone="success"
          disabled={activate.isPending}
          onClick={() => {
            activate.mutate();
          }}
        >
          {t('commerces.detail.reactivate')}
        </TableActionButton>
      ) : null}
    </TableActions>
  );
}

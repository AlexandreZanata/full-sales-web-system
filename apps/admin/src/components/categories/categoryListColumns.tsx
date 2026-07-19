import { ArrowDown, ArrowUp } from 'lucide-react';

import { CategoryThumb } from '@/components/categories/CategoryThumb';
import { ActiveBadge } from '@/components/users/ActiveBadge';
import type { DataTableColumn } from '@/components/ui/DataTable';
import { TableActionButton, TableActions } from '@/components/ui/TableActions';
import type { CategorySummary } from '@/lib/api/types';
import { canMoveCategory } from '@/lib/categories/reorder';
import type { MessageKey } from '@/lib/i18n/messages';

type Translate = (key: MessageKey) => string;

type CategoryListColumnOpts = {
  t: Translate;
  orderedIds: string[];
  catalogRevision: number;
  reorderPending: boolean;
  reactivatePending: boolean;
  onReorder: (categoryId: string, direction: 'up' | 'down') => void;
  onEdit: (row: CategorySummary) => void;
  onDeactivate: (row: CategorySummary) => void;
  onReactivate: (id: string) => void;
};

export function categoryListColumns(
  opts: CategoryListColumnOpts,
): DataTableColumn<CategorySummary>[] {
  const {
    t,
    orderedIds,
    catalogRevision,
    reorderPending,
    reactivatePending,
    onReorder,
    onEdit,
    onDeactivate,
    onReactivate,
  } = opts;

  return [
    {
      id: 'sortOrder',
      header: t('categories.list.sortOrder'),
      cell: (row) => row.sortOrder,
    },
    {
      id: 'name',
      header: t('common.table.name'),
      cell: (row) => (
        <div className="flex items-center gap-3">
          <CategoryThumb
            name={row.name}
            imageFileId={row.imageFileId}
            thumbUrl={row.thumbUrl}
            cacheRevision={catalogRevision}
          />
          <span className="font-medium">{row.name}</span>
        </div>
      ),
    },
    {
      id: 'slug',
      header: t('categories.list.slug'),
      cell: (row) => row.slug,
    },
    {
      id: 'productCount',
      header: t('categories.list.productCount'),
      cell: (row) => row.productCount ?? '—',
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
          <TableActionButton
            tone="neutral"
            disabled={!canMoveCategory(orderedIds, row.id, 'up') || reorderPending}
            aria-label={t('categories.actions.moveUp')}
            onClick={() => {
              onReorder(row.id, 'up');
            }}
          >
            <ArrowUp className="size-4" aria-hidden />
          </TableActionButton>
          <TableActionButton
            tone="neutral"
            disabled={!canMoveCategory(orderedIds, row.id, 'down') || reorderPending}
            aria-label={t('categories.actions.moveDown')}
            onClick={() => {
              onReorder(row.id, 'down');
            }}
          >
            <ArrowDown className="size-4" aria-hidden />
          </TableActionButton>
          <TableActionButton tone="open" onClick={() => onEdit(row)}>
            {t('common.edit')}
          </TableActionButton>
          {row.active ? (
            <TableActionButton tone="danger" onClick={() => onDeactivate(row)}>
              {t('common.deactivate')}
            </TableActionButton>
          ) : (
            <TableActionButton
              tone="success"
              disabled={reactivatePending}
              onClick={() => onReactivate(row.id)}
            >
              {t('categories.actions.reactivate')}
            </TableActionButton>
          )}
        </TableActions>
      ),
    },
  ];
}

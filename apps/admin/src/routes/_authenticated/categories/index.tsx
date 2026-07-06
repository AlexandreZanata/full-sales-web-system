import { createFileRoute } from '@tanstack/react-router';
import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';
import { ArrowDown, ArrowUp } from 'lucide-react';
import { useCallback, useMemo, useState } from 'react';

import { CategoryDialog } from '@/components/categories/CategoryDialog';
import { CategoryThumb } from '@/components/categories/CategoryThumb';
import { ActiveBadge } from '@/components/users/ActiveBadge';
import { Button } from '@/components/ui/Button';
import { ConfirmDialog } from '@/components/ui/ConfirmDialog';
import { DataTable, type DataTableColumn } from '@/components/ui/DataTable';
import { EmptyState } from '@/components/ui/EmptyState';
import { LoadingSpinner } from '@/components/ui/LoadingSpinner';
import { PageHeader } from '@/components/ui/PageHeader';
import { Select } from '@/components/ui/Select';
import {
  deactivateCategory,
  fetchCategories,
  reorderCategories,
  updateCategory,
} from '@/lib/api/categories';
import type { CategorySummary } from '@/lib/api/types';
import { canMoveCategory, moveCategoryInOrder } from '@/lib/categories/reorder';
import { ACTIVE_FILTERS, type ActiveFilter } from '@/lib/commerces/constants';
import { useI18n } from '@/lib/i18n/context';
import { activeFilterLabel } from '@/lib/i18n/labels';
import { useToast } from '@/hooks/useToast';

export const Route = createFileRoute('/_authenticated/categories/')({
  component: CategoriesListPage,
});

function CategoriesListPage() {
  const { t } = useI18n();
  const toast = useToast();
  const queryClient = useQueryClient();
  const [activeFilter, setActiveFilter] = useState<ActiveFilter>('');
  const [dialogOpen, setDialogOpen] = useState(false);
  const [editingCategory, setEditingCategory] = useState<CategorySummary | undefined>();
  const [deactivatingCategory, setDeactivatingCategory] = useState<CategorySummary | undefined>();
  const pageSize = 100;

  const categories = useQuery({
    queryKey: ['categories', activeFilter],
    queryFn: () => fetchCategories({ page: 1, pageSize, active: activeFilter }),
  });

  const reorderMutation = useMutation({
    mutationFn: reorderCategories,
    onSuccess: async () => {
      await queryClient.invalidateQueries({ queryKey: ['categories'] });
      toast.success(t('categories.toast.reordered'));
    },
    onError: () => {
      toast.error(t('errors.actionFailed'));
    },
  });

  const deactivateMutation = useMutation({
    mutationFn: deactivateCategory,
    onSuccess: async () => {
      await queryClient.invalidateQueries({ queryKey: ['categories'] });
      toast.success(t('categories.toast.deactivated'));
      setDeactivatingCategory(undefined);
    },
    onError: () => {
      toast.error(t('errors.actionFailed'));
    },
  });

  const reactivateMutation = useMutation({
    mutationFn: (id: string) => updateCategory(id, { active: true }),
    onSuccess: async () => {
      await queryClient.invalidateQueries({ queryKey: ['categories'] });
      toast.success(t('categories.toast.reactivated'));
    },
    onError: () => {
      toast.error(t('errors.actionFailed'));
    },
  });

  const items = categories.data?.items ?? [];
  const orderedIds = useMemo(() => items.map((item) => item.id), [items]);

  const handleReorder = useCallback(
    async (categoryId: string, direction: 'up' | 'down') => {
      const nextOrder = moveCategoryInOrder(orderedIds, categoryId, direction);
      if (nextOrder.join(',') === orderedIds.join(',')) {
        return;
      }
      await reorderMutation.mutateAsync(nextOrder);
    },
    [orderedIds, reorderMutation],
  );

  const columns: DataTableColumn<CategorySummary>[] = useMemo(
    () => [
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
          <div className="flex flex-wrap justify-end gap-2">
            <Button
              variant="secondary"
              className="h-8 min-h-8 px-2"
              disabled={!canMoveCategory(orderedIds, row.id, 'up') || reorderMutation.isPending}
              aria-label={t('categories.actions.moveUp')}
              onClick={() => {
                handleReorder(row.id, 'up').catch(() => undefined);
              }}
            >
              <ArrowUp className="size-4" aria-hidden />
            </Button>
            <Button
              variant="secondary"
              className="h-8 min-h-8 px-2"
              disabled={!canMoveCategory(orderedIds, row.id, 'down') || reorderMutation.isPending}
              aria-label={t('categories.actions.moveDown')}
              onClick={() => {
                handleReorder(row.id, 'down').catch(() => undefined);
              }}
            >
              <ArrowDown className="size-4" aria-hidden />
            </Button>
            <Button
              variant="secondary"
              className="h-8 min-h-8 px-3 text-xs"
              onClick={() => {
                setEditingCategory(row);
                setDialogOpen(true);
              }}
            >
              {t('common.edit')}
            </Button>
            {row.active ? (
              <Button
                variant="secondary"
                className="h-8 min-h-8 px-3 text-xs"
                onClick={() => {
                  setDeactivatingCategory(row);
                }}
              >
                {t('common.deactivate')}
              </Button>
            ) : (
              <Button
                variant="secondary"
                className="h-8 min-h-8 px-3 text-xs"
                disabled={reactivateMutation.isPending}
                onClick={() => {
                  reactivateMutation.mutate(row.id);
                }}
              >
                {t('categories.actions.reactivate')}
              </Button>
            )}
          </div>
        ),
      },
    ],
    [t, orderedIds, reorderMutation.isPending, reactivateMutation.isPending, handleReorder],
  );

  function openCreateDialog() {
    setEditingCategory(undefined);
    setDialogOpen(true);
  }

  function closeDialog() {
    setDialogOpen(false);
    setEditingCategory(undefined);
  }

  return (
    <div>
      <PageHeader
        title={t('categories.list.title')}
        description={t('categories.list.description')}
        actions={<Button onClick={openCreateDialog}>{t('categories.list.newCategory')}</Button>}
      />

      <div className="mb-4 max-w-xs">
        <Select
          label={t('categories.list.filterByStatus')}
          value={activeFilter}
          onChange={(event) => {
            setActiveFilter(event.target.value as ActiveFilter);
          }}
        >
          {ACTIVE_FILTERS.map((value) => (
            <option key={value || 'all'} value={value}>
              {activeFilterLabel(t, value)}
            </option>
          ))}
        </Select>
      </div>

      {categories.isLoading ? (
        <div className="flex justify-center py-16">
          <LoadingSpinner />
        </div>
      ) : items.length > 0 ? (
        <DataTable
          caption={t('categories.list.caption')}
          columns={columns}
          rows={items}
          getRowKey={(row) => row.id}
          pagination={null}
        />
      ) : (
        <EmptyState
          title={t('categories.list.empty.title')}
          description={
            activeFilter
              ? t('categories.list.empty.descriptionFiltered')
              : t('categories.list.empty.descriptionDefault')
          }
          action={
            activeFilter ? undefined : (
              <Button onClick={openCreateDialog}>{t('categories.list.newCategory')}</Button>
            )
          }
        />
      )}

      <CategoryDialog
        open={dialogOpen}
        category={editingCategory}
        onClose={closeDialog}
        onSaved={() => {
          void queryClient.invalidateQueries({ queryKey: ['categories'] });
        }}
      />

      <ConfirmDialog
        open={Boolean(deactivatingCategory)}
        title={t('categories.deactivateDialog.title')}
        message={t('categories.deactivateDialog.message')}
        confirmLabel={t('categories.deactivateDialog.confirm')}
        destructive
        isLoading={deactivateMutation.isPending}
        onCancel={() => {
          setDeactivatingCategory(undefined);
        }}
        onConfirm={() => {
          if (deactivatingCategory) {
            deactivateMutation.mutate(deactivatingCategory.id);
          }
        }}
      />
    </div>
  );
}

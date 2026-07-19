import { createFileRoute } from '@tanstack/react-router';
import { useQuery, useQueryClient } from '@tanstack/react-query';
import { useCallback, useMemo, useState } from 'react';

import { CategoryDialog } from '@/components/categories/CategoryDialog';
import { categoryListColumns } from '@/components/categories/categoryListColumns';
import { useCategoryListMutations } from '@/components/categories/useCategoryListMutations';
import { Button } from '@/components/ui/Button';
import { ConfirmDialog } from '@/components/ui/ConfirmDialog';
import { DataTable } from '@/components/ui/DataTable';
import { EmptyState } from '@/components/ui/EmptyState';
import { LoadingSpinner } from '@/components/ui/LoadingSpinner';
import { PageHeader } from '@/components/ui/PageHeader';
import { Select } from '@/components/ui/Select';
import { fetchCategories } from '@/lib/api/categories';
import type { CategorySummary } from '@/lib/api/types';
import { moveCategoryInOrder } from '@/lib/categories/reorder';
import { useCatalogRevision } from '@/lib/catalog/useCatalogRevision';
import { ACTIVE_FILTERS, type ActiveFilter } from '@/lib/commerces/constants';
import { useI18n } from '@/lib/i18n/context';
import { activeFilterLabel } from '@/lib/i18n/labels';

export const Route = createFileRoute('/_authenticated/categories/')({
  component: CategoriesListPage,
});

function CategoriesListPage() {
  const { t } = useI18n();
  const queryClient = useQueryClient();
  const catalogRevision = useCatalogRevision();
  const [activeFilter, setActiveFilter] = useState<ActiveFilter>('');
  const [dialogOpen, setDialogOpen] = useState(false);
  const [editingCategory, setEditingCategory] = useState<CategorySummary | undefined>();
  const [deactivatingCategory, setDeactivatingCategory] = useState<CategorySummary | undefined>();

  const categories = useQuery({
    queryKey: ['categories', activeFilter],
    queryFn: () => fetchCategories({ limit: 100, active: activeFilter }),
  });

  const { reorderMutation, deactivateMutation, reactivateMutation } = useCategoryListMutations(
    t,
    () => setDeactivatingCategory(undefined),
  );

  const items = categories.data?.data ?? [];
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

  const columns = useMemo(
    () =>
      categoryListColumns({
        t,
        orderedIds,
        catalogRevision,
        reorderPending: reorderMutation.isPending,
        reactivatePending: reactivateMutation.isPending,
        onReorder: (id, direction) => {
          handleReorder(id, direction).catch(() => undefined);
        },
        onEdit: (row) => {
          setEditingCategory(row);
          setDialogOpen(true);
        },
        onDeactivate: setDeactivatingCategory,
        onReactivate: (id) => reactivateMutation.mutate(id),
      }),
    [t, orderedIds, catalogRevision, reorderMutation.isPending, reactivateMutation, handleReorder],
  );

  function openCreate() {
    setEditingCategory(undefined);
    setDialogOpen(true);
  }

  return (
    <div>
      <PageHeader
        title={t('categories.list.title')}
        description={t('categories.list.description')}
        actions={<Button onClick={openCreate}>{t('categories.list.newCategory')}</Button>}
      />

      <div className="mb-4 max-w-xs">
        <Select
          label={t('categories.list.filterByStatus')}
          value={activeFilter}
          onChange={(event) => setActiveFilter(event.target.value as ActiveFilter)}
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
              <Button onClick={openCreate}>{t('categories.list.newCategory')}</Button>
            )
          }
        />
      )}

      <CategoryDialog
        open={dialogOpen}
        category={editingCategory}
        onClose={() => {
          setDialogOpen(false);
          setEditingCategory(undefined);
        }}
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
        onCancel={() => setDeactivatingCategory(undefined)}
        onConfirm={() => {
          if (deactivatingCategory) {
            deactivateMutation.mutate(deactivatingCategory.id);
          }
        }}
      />
    </div>
  );
}

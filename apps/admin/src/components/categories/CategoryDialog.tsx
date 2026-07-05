import { useQueryClient } from '@tanstack/react-query';
import { useState } from 'react';

import { CategoryForm } from '@/components/categories/CategoryForm';
import { Button } from '@/components/ui/Button';
import { Card } from '@/components/ui/Card';
import { useToast } from '@/hooks/useToast';
import { createCategory, updateCategory } from '@/lib/api/categories';
import type { CategoryDetail, CreateCategoryRequest, UpdateCategoryRequest } from '@/lib/api/types';
import { useI18n } from '@/lib/i18n/context';

type CategoryDialogProps = {
  open: boolean;
  category?: CategoryDetail;
  onClose: () => void;
  onSaved: () => void;
};

export function CategoryDialog({ open, category, onClose, onSaved }: CategoryDialogProps) {
  const { t } = useI18n();
  const toast = useToast();
  const queryClient = useQueryClient();
  const [saved, setSaved] = useState(false);

  if (!open) {
    return null;
  }

  const isEdit = Boolean(category);

  async function handleSubmit(payload: CreateCategoryRequest | UpdateCategoryRequest) {
    try {
      let result: CategoryDetail;
      if (category) {
        result = await updateCategory(category.id, payload);
      } else {
        result = await createCategory(payload as CreateCategoryRequest);
      }
      await queryClient.invalidateQueries({ queryKey: ['categories'] });
      toast.success(isEdit ? t('categories.toast.updated') : t('categories.toast.created'));
      setSaved(true);
      if (isEdit) {
        onSaved();
        onClose();
      }
      return result;
    } catch {
      toast.error(t('errors.actionFailed'));
      throw new Error('Category save failed');
    }
  }

  function handleClose() {
    if (saved && !isEdit) {
      onSaved();
    }
    setSaved(false);
    onClose();
  }

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-foreground/40 p-4">
      <Card
        className="flex max-h-[90vh] w-full max-w-lg flex-col overflow-y-auto p-6"
        role="dialog"
        aria-modal="true"
        aria-labelledby="category-dialog-title"
      >
        <div className="mb-4 flex items-start justify-between gap-4">
          <div>
            <h2 id="category-dialog-title" className="text-lg font-semibold text-foreground">
              {isEdit ? t('categories.edit.title') : t('categories.create.title')}
            </h2>
            <p className="mt-1 text-sm text-muted-foreground">
              {isEdit ? t('categories.edit.description') : t('categories.create.description')}
            </p>
          </div>
          <Button variant="ghost" className="shrink-0" onClick={handleClose}>
            {t('common.cancel')}
          </Button>
        </div>

        <CategoryForm
          key={category?.id ?? 'new'}
          category={category}
          onSubmit={handleSubmit}
          onUpdated={() => {
            if (!isEdit) {
              onSaved();
            }
          }}
          submitLabel={isEdit ? t('categories.form.save') : t('categories.create.submit')}
          submittingLabel={isEdit ? t('categories.form.saving') : t('categories.create.submitting')}
        />
      </Card>
    </div>
  );
}

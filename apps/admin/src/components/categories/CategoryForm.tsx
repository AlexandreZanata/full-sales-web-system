import { useState, type SubmitEvent } from 'react';

import { Button } from '@/components/ui/Button';
import { Input } from '@/components/ui/Input';
import { Textarea } from '@/components/ui/Textarea';
import { CategoryImageSection } from '@/components/categories/CategoryImageSection';
import type { CategoryDetail, CreateCategoryRequest, UpdateCategoryRequest } from '@/lib/api/types';
import {
  categoryToFormValues,
  hasCategoryFormErrors,
  toCreateCategoryPayload,
  toUpdateCategoryPayload,
  validateCategoryForm,
  type CategoryFormValues,
} from '@/lib/categories/validation';
import { useI18n } from '@/lib/i18n/context';
import { translateFormError } from '@/lib/i18n/labels';

type CategoryFormProps = {
  category?: CategoryDetail;
  onSubmit: (payload: CreateCategoryRequest | UpdateCategoryRequest) => Promise<CategoryDetail>;
  onUpdated?: (category: CategoryDetail) => void;
  submitLabel: string;
  submittingLabel: string;
};

const emptyForm: CategoryFormValues = {
  name: '',
  description: '',
  sortOrder: '',
  active: true,
};

export function CategoryForm({
  category,
  onSubmit,
  onUpdated,
  submitLabel,
  submittingLabel,
}: CategoryFormProps) {
  const { t } = useI18n();
  const [values, setValues] = useState<CategoryFormValues>(
    category ? categoryToFormValues(category) : emptyForm,
  );
  const [errors, setErrors] = useState<Partial<Record<keyof CategoryFormValues, string>>>({});
  const [submitting, setSubmitting] = useState(false);
  const [savedCategory, setSavedCategory] = useState<CategoryDetail | undefined>(category);

  function updateField<K extends keyof CategoryFormValues>(key: K, value: CategoryFormValues[K]) {
    setValues((current) => ({ ...current, [key]: value }));
  }

  async function handleSubmit(event: SubmitEvent<HTMLFormElement>) {
    event.preventDefault();
    const nextErrors = validateCategoryForm(values);
    setErrors(nextErrors);
    if (hasCategoryFormErrors(nextErrors)) {
      return;
    }

    setSubmitting(true);
    try {
      const payload = category ? toUpdateCategoryPayload(values) : toCreateCategoryPayload(values);
      const result = await onSubmit(payload);
      setSavedCategory(result);
      onUpdated?.(result);
    } finally {
      setSubmitting(false);
    }
  }

  const imageCategory = savedCategory;

  return (
    <form className="space-y-4" onSubmit={(event) => void handleSubmit(event)}>
      <Input
        label={t('forms.fields.name')}
        name="name"
        value={values.name}
        error={translateFormError(t, errors.name)}
        onChange={(event) => {
          updateField('name', event.target.value);
        }}
      />
      <Textarea
        label={t('categories.form.description')}
        name="description"
        value={values.description}
        onChange={(event) => {
          updateField('description', event.target.value);
        }}
      />
      <div className="grid gap-4 sm:grid-cols-2">
        <Input
          label={t('categories.form.sortOrder')}
          name="sortOrder"
          inputMode="numeric"
          value={values.sortOrder}
          error={translateFormError(t, errors.sortOrder)}
          onChange={(event) => {
            updateField('sortOrder', event.target.value);
          }}
        />
        <label className="flex items-center gap-2 pt-6 text-sm text-foreground">
          <input
            type="checkbox"
            checked={values.active}
            onChange={(event) => {
              updateField('active', event.target.checked);
            }}
          />
          {t('categories.form.active')}
        </label>
      </div>

      {imageCategory ? (
        <CategoryImageSection
          categoryId={imageCategory.id}
          imageFileId={imageCategory.imageFileId}
          onImageUpdated={(fileId) => {
            setSavedCategory({ ...imageCategory, imageFileId: fileId });
          }}
        />
      ) : (
        <p className="text-sm text-muted-foreground">{t('categories.form.imageAfterSave')}</p>
      )}

      <Button type="submit" disabled={submitting}>
        {submitting ? submittingLabel : submitLabel}
      </Button>
    </form>
  );
}

import { useQuery } from '@tanstack/react-query';
import { useState, type SubmitEvent } from 'react';

import { Button } from '@/components/ui/Button';
import { Card } from '@/components/ui/Card';
import { Input } from '@/components/ui/Input';
import { Select } from '@/components/ui/Select';
import { Textarea } from '@/components/ui/Textarea';
import { useToast } from '@/hooks/useToast';
import { fetchCategoriesForPicker } from '@/lib/api/categories';
import type { CreateProductRequest, Product } from '@/lib/api/types';
import { useI18n } from '@/lib/i18n/context';
import { translateFormError } from '@/lib/i18n/labels';
import {
  hasFormErrors,
  toCreateProductPayload,
  validateCreateProductForm,
  type CreateProductFormValues,
} from '@/lib/products/validation';

type CreateProductFormProps = {
  onSubmit: (payload: CreateProductRequest) => Promise<Product>;
  onSuccess: (product: Product) => void;
};

const emptyForm: CreateProductFormValues = {
  name: '',
  sku: '',
  price: '',
  priceCurrency: 'BRL',
  categoryId: '',
  description: '',
};

export function CreateProductForm({ onSubmit, onSuccess }: CreateProductFormProps) {
  const { t } = useI18n();
  const toast = useToast();
  const [values, setValues] = useState<CreateProductFormValues>(emptyForm);
  const [errors, setErrors] = useState<Partial<Record<keyof CreateProductFormValues, string>>>({});
  const [submitting, setSubmitting] = useState(false);

  const categories = useQuery({
    queryKey: ['categories', 'picker'],
    queryFn: fetchCategoriesForPicker,
  });

  function updateField<K extends keyof CreateProductFormValues>(
    key: K,
    value: CreateProductFormValues[K],
  ) {
    setValues((current) => ({ ...current, [key]: value }));
  }

  async function handleSubmit(event: SubmitEvent<HTMLFormElement>) {
    event.preventDefault();
    const nextErrors = validateCreateProductForm(values);
    setErrors(nextErrors);
    if (hasFormErrors(nextErrors)) {
      return;
    }

    setSubmitting(true);
    try {
      const product = await onSubmit(toCreateProductPayload(values));
      onSuccess(product);
    } catch {
      toast.error(t('errors.actionFailed'));
    } finally {
      setSubmitting(false);
    }
  }

  return (
    <Card>
      <form className="space-y-4" onSubmit={(event) => void handleSubmit(event)}>
        <div className="grid gap-4 sm:grid-cols-2">
          <Input
            label={t('forms.fields.name')}
            name="name"
            value={values.name}
            error={translateFormError(t, errors.name)}
            onChange={(event) => {
              updateField('name', event.target.value);
            }}
          />
          <Input
            label={t('forms.fields.sku')}
            name="sku"
            value={values.sku}
            error={translateFormError(t, errors.sku)}
            onChange={(event) => {
              updateField('sku', event.target.value);
            }}
          />
          <Input
            label={t('forms.fields.price')}
            name="price"
            inputMode="decimal"
            placeholder={t('forms.placeholders.price')}
            value={values.price}
            error={translateFormError(t, errors.price)}
            onChange={(event) => {
              updateField('price', event.target.value);
            }}
          />
          <Input
            label={t('forms.fields.currency')}
            name="priceCurrency"
            value={values.priceCurrency}
            onChange={(event) => {
              updateField('priceCurrency', event.target.value.toUpperCase());
            }}
          />
          <Select
            label={t('forms.fields.category')}
            value={values.categoryId}
            onChange={(event) => {
              updateField('categoryId', event.target.value);
            }}
          >
            <option value="">{t('forms.placeholders.selectCategory')}</option>
            {(categories.data ?? []).map((category) => (
              <option key={category.id} value={category.id}>
                {category.name}
              </option>
            ))}
          </Select>
        </div>
        <Textarea
          label={t('forms.fields.description')}
          name="description"
          value={values.description}
          error={translateFormError(t, errors.description)}
          onChange={(event) => {
            updateField('description', event.target.value);
          }}
        />

        <Button type="submit" disabled={submitting}>
          {submitting ? t('products.create.submitting') : t('products.create.submit')}
        </Button>
      </form>
    </Card>
  );
}

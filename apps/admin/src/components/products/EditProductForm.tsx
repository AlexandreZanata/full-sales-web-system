import { useQuery } from '@tanstack/react-query';
import { useState, type SubmitEvent } from 'react';

import { Button } from '@/components/ui/Button';
import { Card } from '@/components/ui/Card';
import { Input } from '@/components/ui/Input';
import { Select } from '@/components/ui/Select';
import { useToast } from '@/hooks/useToast';
import { fetchCategoriesForPicker } from '@/lib/api/categories';
import type { Product } from '@/lib/api/types';
import { useI18n } from '@/lib/i18n/context';
import { translateFormError } from '@/lib/i18n/labels';
import { formatPriceInput } from '@/lib/products/formatPrice';
import {
  hasFormErrors,
  toUpdateProductPayload,
  validateEditProductForm,
  type EditProductFormValues,
} from '@/lib/products/validation';

type EditProductFormProps = {
  product: Product;
  onSubmit: (body: ReturnType<typeof toUpdateProductPayload>) => Promise<Product>;
  onUpdated: (product: Product) => void;
};

export function EditProductForm({ product, onSubmit, onUpdated }: EditProductFormProps) {
  const { t } = useI18n();
  const toast = useToast();
  const [values, setValues] = useState<EditProductFormValues>({
    name: product.name,
    price: formatPriceInput(product.priceAmount),
    priceCurrency: product.priceCurrency,
    unitOfMeasure: product.unitOfMeasure ?? 'Unit',
    categoryId: product.categoryId ?? '',
  });
  const [errors, setErrors] = useState<Partial<Record<keyof EditProductFormValues, string>>>({});
  const [submitting, setSubmitting] = useState(false);

  const categories = useQuery({
    queryKey: ['categories', 'picker'],
    queryFn: fetchCategoriesForPicker,
  });

  async function handleSubmit(event: SubmitEvent<HTMLFormElement>) {
    event.preventDefault();
    const nextErrors = validateEditProductForm(values);
    setErrors(nextErrors);
    if (hasFormErrors(nextErrors)) {
      return;
    }

    setSubmitting(true);
    try {
      const updated = await onSubmit(toUpdateProductPayload(values));
      onUpdated(updated);
      toast.success(t('products.toast.saved'));
    } catch {
      toast.error(t('errors.actionFailed'));
    } finally {
      setSubmitting(false);
    }
  }

  return (
    <Card>
      <form className="space-y-4" onSubmit={(event) => void handleSubmit(event)}>
        <div>
          <Input label={t('forms.fields.sku')} name="sku" value={product.sku} disabled />
          <p className="mt-1 text-xs text-muted-foreground">{t('products.form.skuReadOnly')}</p>
        </div>
        <div className="grid gap-4 sm:grid-cols-2">
          <Input
            label={t('forms.fields.name')}
            name="name"
            value={values.name}
            error={translateFormError(t, errors.name)}
            onChange={(event) => {
              setValues((current) => ({ ...current, name: event.target.value }));
            }}
          />
          <Input
            label={t('forms.fields.price')}
            name="price"
            inputMode="decimal"
            value={values.price}
            error={translateFormError(t, errors.price)}
            onChange={(event) => {
              setValues((current) => ({ ...current, price: event.target.value }));
            }}
          />
          <Select
            label={t('forms.fields.category')}
            value={values.categoryId}
            onChange={(event) => {
              setValues((current) => ({ ...current, categoryId: event.target.value }));
            }}
            disabled={!product.active}
          >
            <option value="">{t('forms.placeholders.selectCategory')}</option>
            {(categories.data ?? []).map((category) => (
              <option key={category.id} value={category.id}>
                {category.name}
              </option>
            ))}
            {product.categoryId &&
            product.categoryName &&
            !(categories.data ?? []).some((category) => category.id === product.categoryId) ? (
              <option value={product.categoryId}>{product.categoryName}</option>
            ) : null}
          </Select>
          <Input
            label={t('forms.fields.unitOfMeasure')}
            name="unitOfMeasure"
            value={values.unitOfMeasure}
            error={translateFormError(t, errors.unitOfMeasure)}
            onChange={(event) => {
              setValues((current) => ({ ...current, unitOfMeasure: event.target.value }));
            }}
          />
        </div>
        <Button type="submit" disabled={submitting || !product.active}>
          {submitting ? t('products.form.saving') : t('products.form.save')}
        </Button>
        {!product.active ? (
          <p className="text-sm text-muted-foreground">{t('products.detail.inactiveHint')}</p>
        ) : null}
      </form>
    </Card>
  );
}

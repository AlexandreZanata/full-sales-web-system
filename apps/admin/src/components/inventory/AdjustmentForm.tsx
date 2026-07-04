import { useQuery } from '@tanstack/react-query';
import { useState, type SubmitEvent } from 'react';

import { Button } from '@/components/ui/Button';
import { Card } from '@/components/ui/Card';
import { Input } from '@/components/ui/Input';
import { Select } from '@/components/ui/Select';
import { Textarea } from '@/components/ui/Textarea';
import { useToast } from '@/hooks/useToast';
import { ApiError } from '@/lib/api/client';
import { recordMovement } from '@/lib/api/inventory';
import { fetchProductsForPicker } from '@/lib/api/products';
import {
  hasFormErrors,
  toAdjustmentPayload,
  validateAdjustmentForm,
  type AdjustmentFormValues,
} from '@/lib/inventory/validation';
import { useI18n } from '@/lib/i18n/context';
import { translateFormError } from '@/lib/i18n/labels';

const emptyForm: AdjustmentFormValues = {
  productId: '',
  quantity: '',
  reason: '',
};

export function AdjustmentForm() {
  const { t } = useI18n();
  const toast = useToast();
  const [values, setValues] = useState<AdjustmentFormValues>(emptyForm);
  const [errors, setErrors] = useState<Partial<Record<keyof AdjustmentFormValues, string>>>({});
  const [submitting, setSubmitting] = useState(false);

  const products = useQuery({
    queryKey: ['products', 'picker'],
    queryFn: fetchProductsForPicker,
  });

  async function handleSubmit(event: SubmitEvent<HTMLFormElement>) {
    event.preventDefault();
    const nextErrors = validateAdjustmentForm(values);
    setErrors(nextErrors);
    if (hasFormErrors(nextErrors)) {
      return;
    }

    setSubmitting(true);
    try {
      await recordMovement(toAdjustmentPayload(values));
      toast.success(t('inventory.toast.adjustmentRecorded'));
      setValues(emptyForm);
    } catch (error) {
      if (error instanceof ApiError && error.code === 'INSUFFICIENT_BALANCE') {
        toast.error(t('errors.actionFailed'));
        return;
      }
      toast.error(t('errors.actionFailed'));
    } finally {
      setSubmitting(false);
    }
  }

  return (
    <Card>
      <form className="space-y-4" onSubmit={(event) => void handleSubmit(event)}>
        <Select
          label={t('forms.fields.product')}
          name="productId"
          value={values.productId}
          error={translateFormError(t, errors.productId)}
          disabled={products.isLoading}
          onChange={(event) => {
            setValues((current) => ({ ...current, productId: event.target.value }));
          }}
        >
          <option value="">{t('forms.placeholders.selectProduct')}</option>
          {products.data?.map((product) => (
            <option key={product.id} value={product.id}>
              {product.sku} — {product.name}
            </option>
          ))}
        </Select>

        <Input
          label={t('forms.fields.quantity')}
          name="quantity"
          type="number"
          step="1"
          value={values.quantity}
          error={translateFormError(t, errors.quantity)}
          onChange={(event) => {
            setValues((current) => ({ ...current, quantity: event.target.value }));
          }}
        />
        <p className="text-xs text-muted-foreground">{t('inventory.adjustments.quantityHint')}</p>

        <Textarea
          label={t('forms.fields.reason')}
          name="reason"
          value={values.reason}
          error={translateFormError(t, errors.reason)}
          onChange={(event) => {
            setValues((current) => ({ ...current, reason: event.target.value }));
          }}
        />

        <Button type="submit" disabled={submitting}>
          {submitting ? t('inventory.adjustments.submitting') : t('inventory.adjustments.submit')}
        </Button>
      </form>
    </Card>
  );
}

import { useQueryClient } from '@tanstack/react-query';
import { useEffect, useState, type SubmitEvent } from 'react';

import { ProductSearchPicker } from '@/components/products/ProductSearchPicker';
import { Button } from '@/components/ui/Button';
import { Card } from '@/components/ui/Card';
import { Input } from '@/components/ui/Input';
import { Textarea } from '@/components/ui/Textarea';
import { useToast } from '@/hooks/useToast';
import { ApiError } from '@/lib/api/client';
import { recordMovement } from '@/lib/api/inventory';
import {
  hasFormErrors,
  toAdjustmentPayload,
  validateAdjustmentForm,
  type AdjustmentDirection,
  type AdjustmentFormValues,
} from '@/lib/inventory/validation';
import { useI18n } from '@/lib/i18n/context';
import { translateFormError } from '@/lib/i18n/labels';
import { cn } from '@/lib/utils';

const emptyForm: AdjustmentFormValues = {
  productId: '',
  direction: 'in',
  quantity: '',
  reason: '',
};

type AdjustmentFormProps = {
  initialProductId?: string;
};

export function AdjustmentForm({ initialProductId }: AdjustmentFormProps) {
  const { t } = useI18n();
  const toast = useToast();
  const queryClient = useQueryClient();
  const [values, setValues] = useState<AdjustmentFormValues>(() => ({
    ...emptyForm,
    productId: initialProductId ?? '',
  }));
  const [errors, setErrors] = useState<Partial<Record<keyof AdjustmentFormValues, string>>>({});
  const [submitting, setSubmitting] = useState(false);

  useEffect(() => {
    if (!initialProductId) {
      return;
    }
    setValues((current) =>
      current.productId === initialProductId
        ? current
        : { ...current, productId: initialProductId },
    );
  }, [initialProductId]);

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
      void queryClient.invalidateQueries({ queryKey: ['inventory', 'balances'] });
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

  function setDirection(direction: AdjustmentDirection) {
    setValues((current) => ({ ...current, direction }));
  }

  return (
    <Card>
      <form className="space-y-4" onSubmit={(event) => void handleSubmit(event)}>
        <ProductSearchPicker
          label={t('forms.fields.product')}
          name="productId"
          value={values.productId}
          error={translateFormError(t, errors.productId)}
          onChange={(productId) => {
            setValues((current) => ({ ...current, productId }));
          }}
        />

        <div className="space-y-1.5">
          <p className="text-xs font-semibold uppercase tracking-[0.12em] text-muted-foreground">
            {t('inventory.adjustments.direction')}
          </p>
          <div className="grid grid-cols-2 gap-2">
            <button
              type="button"
              className={cn(
                'h-10 rounded-md border text-sm font-semibold transition-colors',
                values.direction === 'in'
                  ? 'border-status-active bg-status-active text-white'
                  : 'border-input bg-surface text-foreground hover:bg-surface-muted',
              )}
              aria-pressed={values.direction === 'in'}
              onClick={() => {
                setDirection('in');
              }}
            >
              {t('inventory.adjustments.directionIn')}
            </button>
            <button
              type="button"
              className={cn(
                'h-10 rounded-md border text-sm font-semibold transition-colors',
                values.direction === 'out'
                  ? 'border-destructive bg-destructive text-white'
                  : 'border-input bg-surface text-foreground hover:bg-surface-muted',
              )}
              aria-pressed={values.direction === 'out'}
              onClick={() => {
                setDirection('out');
              }}
            >
              {t('inventory.adjustments.directionOut')}
            </button>
          </div>
        </div>

        <Input
          label={t('forms.fields.quantity')}
          name="quantity"
          type="number"
          min="1"
          step="1"
          inputMode="numeric"
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

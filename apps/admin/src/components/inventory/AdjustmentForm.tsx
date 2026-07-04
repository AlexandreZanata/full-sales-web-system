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
import { formatApiErrorMessage } from '@/lib/utils';

const emptyForm: AdjustmentFormValues = {
  productId: '',
  quantity: '',
  reason: '',
};

export function AdjustmentForm() {
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
      toast.success('Stock adjustment recorded');
      setValues(emptyForm);
    } catch (error) {
      if (error instanceof ApiError && error.code === 'INSUFFICIENT_BALANCE') {
        const message = formatApiErrorMessage(error.message, 'Insufficient stock balance');
        toast.error(message);
        return;
      }
      const message =
        error instanceof ApiError
          ? formatApiErrorMessage(error.message, error.code)
          : 'Unable to record adjustment';
      toast.error(message);
    } finally {
      setSubmitting(false);
    }
  }

  return (
    <Card>
      <form className="space-y-4" onSubmit={(event) => void handleSubmit(event)}>
        <Select
          label="Product"
          name="productId"
          value={values.productId}
          error={errors.productId}
          disabled={products.isLoading}
          onChange={(event) => {
            setValues((current) => ({ ...current, productId: event.target.value }));
          }}
        >
          <option value="">Select product</option>
          {products.data?.map((product) => (
            <option key={product.id} value={product.id}>
              {product.sku} — {product.name}
            </option>
          ))}
        </Select>

        <Input
          label="Quantity"
          name="quantity"
          type="number"
          step="1"
          value={values.quantity}
          error={errors.quantity}
          onChange={(event) => {
            setValues((current) => ({ ...current, quantity: event.target.value }));
          }}
        />
        <p className="text-xs text-muted-foreground">
          Positive adds stock; negative reduces stock. Recorded under your admin account.
        </p>

        <Textarea
          label="Reason"
          name="reason"
          value={values.reason}
          error={errors.reason}
          onChange={(event) => {
            setValues((current) => ({ ...current, reason: event.target.value }));
          }}
        />

        <Button type="submit" disabled={submitting}>
          {submitting ? 'Saving…' : 'Record adjustment'}
        </Button>
      </form>
    </Card>
  );
}

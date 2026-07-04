import { useQuery } from '@tanstack/react-query';
import { useState, type SubmitEvent } from 'react';

import { Button } from '@/components/ui/Button';
import { Card } from '@/components/ui/Card';
import { Input } from '@/components/ui/Input';
import { Select } from '@/components/ui/Select';
import { useToast } from '@/hooks/useToast';
import { ApiError } from '@/lib/api/client';
import { fetchCommercesForPicker } from '@/lib/api/commerces';
import { fetchProductsForPicker } from '@/lib/api/products';
import type { SaleDetail } from '@/lib/api/types';
import { randomId } from '@/lib/randomId';
import { PAYMENT_METHOD_LABELS, PAYMENT_METHODS } from '@/lib/sales/constants';
import { saleActionErrorMessage } from '@/lib/sales/saleActionErrors';
import {
  hasFormErrors,
  toCreateSalePayload,
  validateCreateSaleForm,
  type CreateSaleFormValues,
  type SaleLineFormValues,
} from '@/lib/sales/validation';

const emptyLine = (): SaleLineFormValues => ({ productId: '', quantity: '' });

const emptyForm: CreateSaleFormValues = {
  commerceId: '',
  paymentMethod: '',
  items: [emptyLine()],
};

type CreateSaleFormProps = {
  onSubmit: (
    body: ReturnType<typeof toCreateSalePayload>,
    idempotencyKey: string,
  ) => Promise<SaleDetail>;
  onSuccess: (sale: SaleDetail) => void;
};

export function CreateSaleForm({ onSubmit, onSuccess }: CreateSaleFormProps) {
  const toast = useToast();
  const [values, setValues] = useState<CreateSaleFormValues>(emptyForm);
  const [errors, setErrors] = useState<ReturnType<typeof validateCreateSaleForm>>({});
  const [submitting, setSubmitting] = useState(false);

  const commerces = useQuery({
    queryKey: ['commerces', 'picker'],
    queryFn: fetchCommercesForPicker,
  });

  const products = useQuery({
    queryKey: ['products', 'picker'],
    queryFn: fetchProductsForPicker,
  });

  async function handleSubmit(event: SubmitEvent<HTMLFormElement>) {
    event.preventDefault();
    const nextErrors = validateCreateSaleForm(values);
    setErrors(nextErrors);
    if (hasFormErrors(nextErrors)) {
      return;
    }

    setSubmitting(true);
    try {
      const sale = await onSubmit(toCreateSalePayload(values), randomId());
      toast.success('Sale created');
      onSuccess(sale);
    } catch (error) {
      const message =
        error instanceof ApiError ? saleActionErrorMessage(error.code) : 'Unable to create sale';
      toast.error(message);
    } finally {
      setSubmitting(false);
    }
  }

  function updateLine(index: number, patch: Partial<SaleLineFormValues>) {
    setValues((current) => ({
      ...current,
      items: current.items.map((line, i) => (i === index ? { ...line, ...patch } : line)),
    }));
  }

  return (
    <Card>
      <form className="space-y-4" onSubmit={(event) => void handleSubmit(event)}>
        <Select
          label="Commerce"
          value={values.commerceId}
          error={errors.commerceId}
          disabled={commerces.isLoading}
          onChange={(event) => {
            setValues((current) => ({ ...current, commerceId: event.target.value }));
          }}
        >
          <option value="">Select commerce</option>
          {(commerces.data ?? []).map((commerce) => (
            <option key={commerce.id} value={commerce.id}>
              {commerce.tradeName || commerce.legalName}
            </option>
          ))}
        </Select>

        <Select
          label="Payment method"
          value={values.paymentMethod}
          error={errors.paymentMethod}
          onChange={(event) => {
            setValues((current) => ({
              ...current,
              paymentMethod: event.target.value as CreateSaleFormValues['paymentMethod'],
            }));
          }}
        >
          <option value="">Select payment method</option>
          {PAYMENT_METHODS.map((method) => (
            <option key={method} value={method}>
              {PAYMENT_METHOD_LABELS[method]}
            </option>
          ))}
        </Select>

        <div className="space-y-3">
          <p className="text-xs font-semibold uppercase tracking-[0.12em] text-muted-foreground">
            Line items
          </p>
          {values.items.map((line, index) => (
            <div key={index} className="grid gap-3 sm:grid-cols-[1fr_8rem_auto]">
              <Select
                label={index === 0 ? 'Product' : undefined}
                value={line.productId}
                error={errors.items?.[index]?.productId}
                disabled={products.isLoading}
                onChange={(event) => {
                  updateLine(index, { productId: event.target.value });
                }}
              >
                <option value="">Select product</option>
                {(products.data ?? []).map((product) => (
                  <option key={product.id} value={product.id}>
                    {product.sku} — {product.name}
                  </option>
                ))}
              </Select>
              <Input
                label={index === 0 ? 'Qty' : undefined}
                type="number"
                min={1}
                value={line.quantity}
                error={errors.items?.[index]?.quantity}
                onChange={(event) => {
                  updateLine(index, { quantity: event.target.value });
                }}
              />
              <div className={index === 0 ? 'pt-6' : 'pt-0 sm:pt-6'}>
                <Button
                  type="button"
                  variant="secondary"
                  disabled={values.items.length <= 1}
                  onClick={() => {
                    setValues((current) => ({
                      ...current,
                      items: current.items.filter((_, i) => i !== index),
                    }));
                  }}
                >
                  Remove
                </Button>
              </div>
            </div>
          ))}
          <Button
            type="button"
            variant="secondary"
            onClick={() => {
              setValues((current) => ({
                ...current,
                items: [...current.items, emptyLine()],
              }));
            }}
          >
            Add line
          </Button>
        </div>

        <p className="text-xs text-muted-foreground">
          Sales are recorded under your admin account as the responsible driver (API limitation).
        </p>

        <Button type="submit" disabled={submitting}>
          {submitting ? 'Creating…' : 'Create sale'}
        </Button>
      </form>
    </Card>
  );
}

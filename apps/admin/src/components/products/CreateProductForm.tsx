import { useState, type SubmitEvent } from 'react';

import { Button } from '@/components/ui/Button';
import { Card } from '@/components/ui/Card';
import { Input } from '@/components/ui/Input';
import { useToast } from '@/hooks/useToast';
import { ApiError } from '@/lib/api/client';
import type { CreateProductRequest, Product } from '@/lib/api/types';
import {
  hasFormErrors,
  toCreateProductPayload,
  validateCreateProductForm,
  type CreateProductFormValues,
} from '@/lib/products/validation';
import { formatApiErrorMessage } from '@/lib/utils';

type CreateProductFormProps = {
  onSubmit: (payload: CreateProductRequest) => Promise<Product>;
  onSuccess: (product: Product) => void;
};

const emptyForm: CreateProductFormValues = {
  name: '',
  sku: '',
  price: '',
  priceCurrency: 'BRL',
};

export function CreateProductForm({ onSubmit, onSuccess }: CreateProductFormProps) {
  const toast = useToast();
  const [values, setValues] = useState<CreateProductFormValues>(emptyForm);
  const [errors, setErrors] = useState<Partial<Record<keyof CreateProductFormValues, string>>>({});
  const [submitting, setSubmitting] = useState(false);

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
    } catch (error) {
      const message =
        error instanceof ApiError
          ? formatApiErrorMessage(error.message, error.code)
          : 'Unable to create product';
      toast.error(message);
    } finally {
      setSubmitting(false);
    }
  }

  return (
    <Card>
      <form className="space-y-4" onSubmit={(event) => void handleSubmit(event)}>
        <div className="grid gap-4 sm:grid-cols-2">
          <Input
            label="Name"
            name="name"
            value={values.name}
            error={errors.name}
            onChange={(event) => {
              updateField('name', event.target.value);
            }}
          />
          <Input
            label="SKU"
            name="sku"
            value={values.sku}
            error={errors.sku}
            onChange={(event) => {
              updateField('sku', event.target.value);
            }}
          />
          <Input
            label="Price"
            name="price"
            inputMode="decimal"
            placeholder="0,00"
            value={values.price}
            error={errors.price}
            onChange={(event) => {
              updateField('price', event.target.value);
            }}
          />
          <Input
            label="Currency"
            name="priceCurrency"
            value={values.priceCurrency}
            onChange={(event) => {
              updateField('priceCurrency', event.target.value.toUpperCase());
            }}
          />
        </div>

        <Button type="submit" disabled={submitting}>
          {submitting ? 'Creating…' : 'Create product'}
        </Button>
      </form>
    </Card>
  );
}

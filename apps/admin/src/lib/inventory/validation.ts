export type AdjustmentDirection = 'in' | 'out';

export type AdjustmentFormValues = {
  productId: string;
  direction: AdjustmentDirection;
  quantity: string;
  reason: string;
};

export type FormErrors<T extends string> = Partial<Record<T, string>>;

export function validateAdjustmentForm(
  values: AdjustmentFormValues,
): FormErrors<keyof AdjustmentFormValues> {
  const errors: FormErrors<keyof AdjustmentFormValues> = {};

  if (!values.productId) {
    errors.productId = 'forms.validation.selectProduct';
  }

  const quantity = Number.parseInt(values.quantity, 10);
  if (!values.quantity.trim() || Number.isNaN(quantity) || quantity <= 0) {
    errors.quantity = 'forms.validation.quantityAdjustment';
  }

  if (!values.reason.trim()) {
    errors.reason = 'forms.validation.reasonRequired';
  }

  return errors;
}

export function hasFormErrors<T extends string>(errors: FormErrors<T>): boolean {
  return Object.keys(errors).length > 0;
}

export function toAdjustmentPayload(values: AdjustmentFormValues) {
  const absolute = Number.parseInt(values.quantity, 10);
  return {
    productId: values.productId,
    movementType: 'Adjustment' as const,
    quantity: values.direction === 'out' ? -absolute : absolute,
    reason: values.reason.trim(),
  };
}

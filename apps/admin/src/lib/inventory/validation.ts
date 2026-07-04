export type AdjustmentFormValues = {
  productId: string;
  quantity: string;
  reason: string;
};

export type FormErrors<T extends string> = Partial<Record<T, string>>;

export function validateAdjustmentForm(
  values: AdjustmentFormValues,
): FormErrors<keyof AdjustmentFormValues> {
  const errors: FormErrors<keyof AdjustmentFormValues> = {};

  if (!values.productId) {
    errors.productId = 'Select a product';
  }

  const quantity = Number.parseInt(values.quantity, 10);
  if (!values.quantity.trim() || Number.isNaN(quantity) || quantity === 0) {
    errors.quantity = 'Enter a non-zero quantity (negative reduces stock)';
  }

  if (!values.reason.trim()) {
    errors.reason = 'Reason is required';
  }

  return errors;
}

export function hasFormErrors<T extends string>(errors: FormErrors<T>): boolean {
  return Object.keys(errors).length > 0;
}

export function toAdjustmentPayload(values: AdjustmentFormValues) {
  return {
    productId: values.productId,
    movementType: 'Adjustment' as const,
    quantity: Number.parseInt(values.quantity, 10),
    reason: values.reason.trim(),
  };
}

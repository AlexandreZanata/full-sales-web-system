/** Contract: RN10 — rejection reason required (non-empty after trim). */
export type RejectOrderFormValues = {
  reason: string;
};

export type RejectOrderFormErrors = Partial<Record<keyof RejectOrderFormValues, string>>;

export function validateRejectOrderForm(values: RejectOrderFormValues): RejectOrderFormErrors {
  const errors: RejectOrderFormErrors = {};
  if (!values.reason.trim()) {
    errors.reason = 'Rejection reason is required';
  }
  return errors;
}

export function hasFormErrors(errors: RejectOrderFormErrors): boolean {
  return Object.keys(errors).length > 0;
}

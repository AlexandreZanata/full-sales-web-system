/**
 * Contract: Phase 34 create sale form — commerce, items, payment validation.
 */
import { describe, expect, it } from 'vitest';

import {
  hasFormErrors,
  toCreateSalePayload,
  validateCreateSaleForm,
  type CreateSaleFormValues,
} from '@/lib/sales/validation';

const validForm: CreateSaleFormValues = {
  commerceId: '550e8400-e29b-41d4-a716-446655440000',
  paymentMethod: 'cash',
  items: [{ productId: '660e8400-e29b-41d4-a716-446655440001', quantity: '2' }],
};

describe('validateCreateSaleForm — Phase 34 contract', () => {
  it('given_valid_form_when_validate_then_no_errors', () => {
    expect(hasFormErrors(validateCreateSaleForm(validForm))).toBe(false);
  });

  it('given_missing_commerce_when_validate_then_commerce_error', () => {
    const errors = validateCreateSaleForm({ ...validForm, commerceId: '' });
    expect(errors.commerceId).toBe('forms.validation.selectCommerce');
  });

  it('given_invalid_payment_when_validate_then_payment_error', () => {
    const errors = validateCreateSaleForm({ ...validForm, paymentMethod: '' });
    expect(errors.paymentMethod).toBe('forms.validation.selectPaymentMethod');
  });

  it('given_empty_items_when_validate_then_has_errors', () => {
    const errors = validateCreateSaleForm({
      ...validForm,
      items: [{ productId: '', quantity: '' }],
    });
    expect(errors.itemsRoot).toBe('forms.validation.itemsRequired');
    expect(hasFormErrors(errors)).toBe(true);
  });
});

describe('toCreateSalePayload — API contract', () => {
  it('given_valid_form_when_to_payload_then_lowercase_payment_method', () => {
    const payload = toCreateSalePayload(validForm);
    expect(payload.paymentMethod).toBe('cash');
    expect(payload.items).toHaveLength(1);
    expect(payload.items[0]?.quantity).toBe(2);
  });
});

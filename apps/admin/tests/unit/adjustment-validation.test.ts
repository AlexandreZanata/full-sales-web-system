/**
 * Contract: adjustment form uses Entrada/Saída; API quantity is signed on submit only.
 */
import { describe, expect, it } from 'vitest';

import {
  hasFormErrors,
  toAdjustmentPayload,
  validateAdjustmentForm,
} from '@/lib/inventory/validation';

const base = {
  productId: '550e8400-e29b-41d4-a716-446655440000',
  direction: 'in' as const,
  quantity: '5',
  reason: 'Stock correction',
};

describe('validateAdjustmentForm — Phase 32 contract', () => {
  it('given_valid_form_when_validate_then_no_errors', () => {
    expect(hasFormErrors(validateAdjustmentForm(base))).toBe(false);
  });

  it('given_zero_quantity_when_validate_then_quantity_error', () => {
    const errors = validateAdjustmentForm({ ...base, quantity: '0' });
    expect(errors.quantity).toBe('forms.validation.quantityAdjustment');
  });

  it('given_negative_quantity_when_validate_then_quantity_error', () => {
    const errors = validateAdjustmentForm({ ...base, quantity: '-5' });
    expect(errors.quantity).toBe('forms.validation.quantityAdjustment');
  });

  it('given_empty_reason_when_validate_then_reason_required', () => {
    const errors = validateAdjustmentForm({ ...base, reason: '  ' });
    expect(errors.reason).toBe('forms.validation.reasonRequired');
  });
});

describe('toAdjustmentPayload — API contract', () => {
  it('given_entrada_when_to_payload_then_positive_quantity', () => {
    expect(toAdjustmentPayload({ ...base, direction: 'in', quantity: '10' })).toEqual({
      productId: base.productId,
      movementType: 'Adjustment',
      quantity: 10,
      reason: 'Stock correction',
    });
  });

  it('given_saida_when_to_payload_then_negative_quantity', () => {
    expect(toAdjustmentPayload({ ...base, direction: 'out', quantity: '10', reason: 'Damaged' })).toEqual({
      productId: base.productId,
      movementType: 'Adjustment',
      quantity: -10,
      reason: 'Damaged',
    });
  });
});

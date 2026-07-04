/**
 * Contract: Phase 32 adjustment form — reason required, INSUFFICIENT_BALANCE payload shape.
 */
import { describe, expect, it } from 'vitest';

import {
  hasFormErrors,
  toAdjustmentPayload,
  validateAdjustmentForm,
} from '@/lib/inventory/validation';

describe('validateAdjustmentForm — Phase 32 contract', () => {
  it('given_valid_form_when_validate_then_no_errors', () => {
    const errors = validateAdjustmentForm({
      productId: '550e8400-e29b-41d4-a716-446655440000',
      quantity: '-5',
      reason: 'Stock correction',
    });
    expect(hasFormErrors(errors)).toBe(false);
  });

  it('given_zero_quantity_when_validate_then_quantity_error', () => {
    const errors = validateAdjustmentForm({
      productId: '550e8400-e29b-41d4-a716-446655440000',
      quantity: '0',
      reason: 'Correction',
    });
    expect(errors.quantity).toBe('Enter a non-zero quantity (negative reduces stock)');
  });

  it('given_empty_reason_when_validate_then_reason_required', () => {
    const errors = validateAdjustmentForm({
      productId: '550e8400-e29b-41d4-a716-446655440000',
      quantity: '5',
      reason: '  ',
    });
    expect(errors.reason).toBe('Reason is required');
  });
});

describe('toAdjustmentPayload — API contract', () => {
  it('given_valid_form_when_to_payload_then_adjustment_type', () => {
    const payload = toAdjustmentPayload({
      productId: '550e8400-e29b-41d4-a716-446655440000',
      quantity: '-3',
      reason: 'Damaged units',
    });
    expect(payload).toEqual({
      productId: '550e8400-e29b-41d4-a716-446655440000',
      movementType: 'Adjustment',
      quantity: -3,
      reason: 'Damaged units',
    });
  });
});

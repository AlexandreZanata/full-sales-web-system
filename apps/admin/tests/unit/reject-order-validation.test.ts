/**
 * Contract: Phase 33 reject order — RN10 rejection reason required (client-side).
 */
import { describe, expect, it } from 'vitest';

import { hasFormErrors, validateRejectOrderForm } from '@/lib/orders/validation';

describe('validateRejectOrderForm — RN10 contract', () => {
  it('given_empty_reason_when_validate_then_required_error', () => {
    const errors = validateRejectOrderForm({ reason: '' });
    expect(errors.reason).toBe('Rejection reason is required');
    expect(hasFormErrors(errors)).toBe(true);
  });

  it('given_whitespace_reason_when_validate_then_required_error', () => {
    const errors = validateRejectOrderForm({ reason: '   ' });
    expect(errors.reason).toBe('Rejection reason is required');
  });

  it('given_non_empty_reason_when_validate_then_no_errors', () => {
    const errors = validateRejectOrderForm({ reason: 'Out of delivery zone' });
    expect(hasFormErrors(errors)).toBe(false);
  });
});

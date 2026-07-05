/**
 * Contract: Phase 50 — sales contact phone validation.
 */
import { describe, expect, it } from 'vitest';

import {
  isValidSalesContactPhone,
  normalizeSalesContactPhone,
  validateSalesContactPhone,
} from '@/lib/settings/phone';

describe('sales contact phone — Phase 50 contract', () => {
  it('given_br_mobile_when_normalize_then_digits_only', () => {
    expect(normalizeSalesContactPhone('+55 (11) 98765-4321')).toBe('5511987654321');
  });

  it('given_empty_when_validate_then_no_error', () => {
    expect(validateSalesContactPhone('')).toBeUndefined();
  });

  it('given_short_phone_when_validate_then_invalid_key', () => {
    expect(validateSalesContactPhone('123')).toBe('settings.validation.salesContactPhoneInvalid');
  });

  it('given_valid_digits_when_validate_then_passes', () => {
    expect(isValidSalesContactPhone('11987654321')).toBe(true);
    expect(validateSalesContactPhone('11987654321')).toBeUndefined();
  });
});

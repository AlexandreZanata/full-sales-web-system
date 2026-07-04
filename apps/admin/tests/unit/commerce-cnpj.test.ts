/**
 * Contract: BR-CO-001 — CNPJ validation and formatting for admin commerce forms.
 */
import { describe, expect, it } from 'vitest';

import { formatCnpj, isValidCnpj, stripCnpj } from '@/lib/commerces/cnpj';

const VALID_CNPJ = '11444777000161';

describe('isValidCnpj — BR-CO-001 contract', () => {
  it('given_valid_cnpj_when_validate_then_true', () => {
    expect(isValidCnpj(VALID_CNPJ)).toBe(true);
  });

  it('given_formatted_cnpj_when_validate_then_true', () => {
    expect(isValidCnpj('11.444.777/0001-61')).toBe(true);
  });

  it('given_invalid_check_digits_when_validate_then_false', () => {
    expect(isValidCnpj('11444777000162')).toBe(false);
  });

  it('given_all_same_digits_when_validate_then_false', () => {
    expect(isValidCnpj('11111111111111')).toBe(false);
  });
});

describe('formatCnpj — display contract', () => {
  it('given_digits_when_format_then_brazilian_mask', () => {
    expect(formatCnpj(VALID_CNPJ)).toBe('11.444.777/0001-61');
  });

  it('given_strip_when_called_then_digits_only', () => {
    expect(stripCnpj('11.444.777/0001-61')).toBe(VALID_CNPJ);
  });
});

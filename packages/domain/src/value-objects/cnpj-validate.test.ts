import { describe, expect, it } from 'vitest';

import { isValidCnpj } from './cnpj-validate.js';

describe('isValidCnpj', () => {
  it('given_known_valid_cnpj_when_validate_then_true', () => {
    expect(isValidCnpj('11444777000161')).toBe(true);
  });

  it('given_all_zeros_when_validate_then_false', () => {
    expect(isValidCnpj('00000000000000')).toBe(false);
  });
});

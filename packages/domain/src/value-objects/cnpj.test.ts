import { describe, expect, it } from 'vitest';

import { InvalidCnpjError } from '../errors/domain-error.js';
import { Cnpj } from './cnpj.js';

const VALID_CNPJ = '11444777000161';

describe('Cnpj', () => {
  // Contract: BR-CO-001 — invalid CNPJ rejected at creation.
  it('BR-CO-001_given_invalid_check_digits_when_parse_then_invalid_cnpj', () => {
    expect(() => Cnpj.parse('00000000000000')).toThrow(InvalidCnpjError);
  });

  it('given_valid_cnpj_when_parse_then_stores_digits', () => {
    const cnpj = Cnpj.parse(VALID_CNPJ);
    expect(cnpj.toString()).toBe(VALID_CNPJ);
  });

  it('given_formatted_cnpj_when_parse_then_normalizes', () => {
    const cnpj = Cnpj.parse('11.444.777/0001-61');
    expect(cnpj.toString()).toBe(VALID_CNPJ);
  });
});

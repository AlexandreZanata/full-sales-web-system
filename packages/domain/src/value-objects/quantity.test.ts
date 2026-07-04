import { describe, expect, it } from 'vitest';

import { InvalidQuantityError } from '../errors/domain-error.js';
import { Quantity } from './quantity.js';

describe('Quantity', () => {
  it('given_positive_integer_when_create_then_ok', () => {
    expect(Quantity.of(1).value).toBe(1);
  });

  it('given_zero_when_create_then_invalid_quantity', () => {
    expect(() => Quantity.of(0)).toThrow(InvalidQuantityError);
  });

  it('given_negative_when_create_then_invalid_quantity', () => {
    expect(() => Quantity.of(-2)).toThrow(InvalidQuantityError);
  });

  it('given_fraction_when_create_then_invalid_quantity', () => {
    expect(() => Quantity.of(1.5)).toThrow(InvalidQuantityError);
  });
});

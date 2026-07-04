/**
 * Contract: Phase 32 price formatting — minor units to BRL display.
 */
import { describe, expect, it } from 'vitest';

import { formatMoney, formatPriceInput, parsePriceInput } from '@/lib/products/formatPrice';

describe('parsePriceInput — Phase 32 contract', () => {
  it('given_brazilian_decimal_when_parse_then_centavos', () => {
    expect(parsePriceInput('25,50')).toBe(2550);
  });

  it('given_invalid_price_when_parse_then_null', () => {
    expect(parsePriceInput('abc')).toBeNull();
  });
});

describe('formatMoney — display contract', () => {
  it('given_centavos_when_format_then_brl_currency', () => {
    expect(formatMoney(2500, 'BRL')).toContain('25');
  });
});

describe('formatPriceInput — edit form contract', () => {
  it('given_centavos_when_format_then_comma_decimal', () => {
    expect(formatPriceInput(2550)).toBe('25,50');
  });
});

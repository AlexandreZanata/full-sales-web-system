import { describe, expect, it } from 'vitest';

import { CurrencyMismatchError, NegativeMoneyAmountError } from '../errors/domain-error.js';
import { Currency } from './currency.js';
import { Money } from './money.js';

describe('Money', () => {
  // Contract: GLOSSARY — Money uses minor units, never floating point.
  it('given_valid_brl_minor_units_when_create_then_ok', () => {
    const money = Money.of(15_000, Currency.brl());
    expect(money.amountMinor).toBe(15_000);
    expect(money.currency.toString()).toBe('BRL');
  });

  it('given_negative_amount_when_create_then_negative_money_amount', () => {
    expect(() => Money.of(-1, Currency.brl())).toThrow(NegativeMoneyAmountError);
  });

  // Contract: BR-SA-002 — total computed from items only (10000 + 5000 = 15000).
  it('BR-SA-002_given_two_line_items_when_add_then_total_from_items', () => {
    const lineA = Money.of(10_000, Currency.brl());
    const lineB = Money.of(5_000, Currency.brl());
    expect(lineA.add(lineB).amountMinor).toBe(15_000);
  });

  it('given_different_currencies_when_add_then_currency_mismatch', () => {
    const brl = Money.of(100, Currency.brl());
    const usd = Money.of(100, Currency.parse('USD'));
    expect(() => brl.add(usd)).toThrow(CurrencyMismatchError);
  });
});

describe('Currency', () => {
  it('given_invalid_currency_code_when_parse_then_invalid_currency', () => {
    expect(() => Currency.parse('brl')).toThrow();
    expect(() => Currency.parse('US')).toThrow();
  });

  it('given_valid_currency_code_when_parse_then_ok', () => {
    expect(Currency.parse('USD').toString()).toBe('USD');
  });
});

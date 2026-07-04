import {
  CurrencyMismatchError,
  MoneyOverflowError,
  NegativeMoneyAmountError,
} from '../errors/domain-error.js';
import { Currency } from './currency.js';

/** Monetary amount in minor units with currency — never floating point. */
export class Money {
  private constructor(
    readonly amountMinor: number,
    readonly currency: Currency,
  ) {}

  static of(amountMinor: number, currency: Currency): Money {
    if (amountMinor < 0) {
      throw new NegativeMoneyAmountError();
    }
    if (!Number.isInteger(amountMinor)) {
      throw new NegativeMoneyAmountError();
    }
    return new Money(amountMinor, currency);
  }

  add(other: Money): Money {
    if (!this.currency.equals(other.currency)) {
      throw new CurrencyMismatchError(this.currency.toString(), other.currency.toString());
    }
    const sum = this.amountMinor + other.amountMinor;
    if (sum > Number.MAX_SAFE_INTEGER) {
      throw new MoneyOverflowError();
    }
    return Money.of(sum, this.currency);
  }

  multiply(factor: number): Money {
    if (!Number.isInteger(factor) || factor < 0) {
      throw new NegativeMoneyAmountError();
    }
    const product = this.amountMinor * factor;
    if (product > Number.MAX_SAFE_INTEGER) {
      throw new MoneyOverflowError();
    }
    return Money.of(product, this.currency);
  }
}

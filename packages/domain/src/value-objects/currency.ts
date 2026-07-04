import { InvalidCurrencyError } from '../errors/domain-error.js';

/** ISO 4217 currency code (3 uppercase ASCII letters). */
export class Currency {
  private constructor(private readonly code: string) {}

  static parse(code: string): Currency {
    if (code.length !== 3 || !/^[A-Z]{3}$/.test(code)) {
      throw new InvalidCurrencyError();
    }
    return new Currency(code);
  }

  static brl(): Currency {
    return new Currency('BRL');
  }

  toString(): string {
    return this.code;
  }

  equals(other: Currency): boolean {
    return this.code === other.code;
  }
}

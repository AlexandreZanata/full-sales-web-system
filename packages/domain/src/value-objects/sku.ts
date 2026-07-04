import { InvalidSkuError } from '../errors/domain-error.js';

/** Stock-keeping unit identifier for a Product. */
export class Sku {
  private constructor(readonly value: string) {}

  static parse(value: string): Sku {
    const trimmed = value.trim();
    if (!/^[A-Za-z0-9._-]{1,64}$/.test(trimmed)) {
      throw new InvalidSkuError();
    }
    return new Sku(trimmed);
  }

  toString(): string {
    return this.value;
  }
}

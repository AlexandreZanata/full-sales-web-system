import { InvalidQuantityError } from '../errors/domain-error.js';

/** Positive integer quantity for sale lines and stock. */
export class Quantity {
  private constructor(readonly value: number) {}

  static of(value: number): Quantity {
    if (!Number.isInteger(value) || value < 1) {
      throw new InvalidQuantityError();
    }
    return new Quantity(value);
  }
}

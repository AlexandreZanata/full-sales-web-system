import type { ProductId } from '../value-objects/ids.js';
import { Money } from '../value-objects/money.js';
import { Quantity } from '../value-objects/quantity.js';

/** Single product line on a Sale — quantity, unit price, line subtotal. */
export class SaleItem {
  private constructor(
    readonly productId: ProductId,
    readonly quantity: Quantity,
    readonly unitPrice: Money,
    readonly lineTotal: Money,
  ) {}

  static create(productId: ProductId, quantity: Quantity, unitPrice: Money): SaleItem {
    const lineTotal = unitPrice.multiply(quantity.value);
    return new SaleItem(productId, quantity, unitPrice, lineTotal);
  }
}

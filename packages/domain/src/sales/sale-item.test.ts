import { describe, expect, it } from 'vitest';

import { Currency } from '../value-objects/currency.js';
import { Quantity } from '../value-objects/quantity.js';
import { SaleItem } from './sale-item.js';
import { parseProductId } from '../value-objects/ids.js';
import { Money } from '../value-objects/money.js';

describe('SaleItem', () => {
  const productId = parseProductId('550e8400-e29b-41d4-a716-446655440001');
  const unitPrice = Money.of(2_500, Currency.brl());

  // Contract: BR-SA-002 / glossary — line total = quantity × unit price.
  it('BR-SA-002_given_quantity_and_unit_price_when_create_then_line_total_is_product', () => {
    const item = SaleItem.create(productId, Quantity.of(2), unitPrice);
    expect(item.lineTotal.amountMinor).toBe(5_000);
  });

  it('given_minimum_quantity_when_create_then_ok', () => {
    const item = SaleItem.create(productId, Quantity.of(1), unitPrice);
    expect(item.quantity.value).toBe(1);
  });
});

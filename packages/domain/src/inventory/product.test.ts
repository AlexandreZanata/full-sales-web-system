import { describe, expect, it } from 'vitest';

import { Product } from './product.js';
import { Currency } from '../value-objects/currency.js';
import { parseProductId, parseTenantId } from '../value-objects/ids.js';
import { Money } from '../value-objects/money.js';
import { Sku } from '../value-objects/sku.js';

const tenantId = parseTenantId('550e8400-e29b-41d4-a716-446655440000');
const productId = parseProductId('550e8400-e29b-41d4-a716-446655440003');

describe('Product', () => {
  it('given_valid_input_when_create_then_active', () => {
    const product = Product.create({
      id: productId,
      name: 'Widget',
      sku: Sku.parse('WGT-001'),
      unitPrice: Money.of(500, Currency.brl()),
      tenantId,
    });
    expect(product.isActive()).toBe(true);
    expect(product.name).toBe('Widget');
  });

  it('given_active_product_when_deactivate_then_inactive', () => {
    const product = Product.create({
      id: productId,
      name: 'Widget',
      sku: Sku.parse('WGT-001'),
      unitPrice: Money.of(500, Currency.brl()),
      tenantId,
    });
    expect(product.deactivate().isActive()).toBe(false);
  });
});

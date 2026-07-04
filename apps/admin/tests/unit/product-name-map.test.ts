/**
 * Contract: product line-item display names from picker data.
 */
import { describe, expect, it } from 'vitest';

import { buildProductNameMap, productDisplayName } from '@/lib/products/productNameMap';

describe('productNameMap — follow-up GAP-056/064', () => {
  it('given_products_when_build_map_then_sku_and_name', () => {
    const map = buildProductNameMap([
      {
        id: '550e8400-e29b-41d4-a716-446655440000',
        sku: 'SKU-1',
        name: 'Widget',
        priceAmount: 1000,
        priceCurrency: 'BRL',
        active: true,
      },
    ]);
    expect(productDisplayName(map, '550e8400-e29b-41d4-a716-446655440000')).toBe('SKU-1 — Widget');
  });

  it('given_unknown_id_when_display_then_truncated_uuid', () => {
    const map = buildProductNameMap([]);
    expect(productDisplayName(map, '660e8400-e29b-41d4-a716-446655440001')).toBe('660e8400…');
  });
});

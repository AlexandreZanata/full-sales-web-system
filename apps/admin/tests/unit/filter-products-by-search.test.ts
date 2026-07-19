/**
 * Contract: product picker search matches SKU or name (case-insensitive substring).
 */
import { describe, expect, it } from 'vitest';

import type { ProductSummary } from '@/lib/api/types';
import {
  filterProductsBySearch,
  formatProductOption,
} from '@/lib/products/filterProductsBySearch';

function product(partial: Pick<ProductSummary, 'id' | 'sku' | 'name'>): ProductSummary {
  return {
    ...partial,
    priceAmount: 100,
    priceCurrency: 'BRL',
    active: true,
  };
}

const catalog = [
  product({ id: '1', sku: 'SEED-003', name: 'Água Crystal 500ml' }),
  product({ id: '2', sku: 'SEED-010', name: 'Refrigerante Cola 2L' }),
  product({ id: '3', sku: 'SKU-99', name: 'Snack Pack' }),
];

describe('filterProductsBySearch', () => {
  it('given_sku_prefix_when_search_then_matching_products', () => {
    const result = filterProductsBySearch(catalog, 'SEED');
    expect(result.map((item) => item.id)).toEqual(['1', '2']);
  });

  it('given_name_substring_when_search_then_case_insensitive_match', () => {
    const result = filterProductsBySearch(catalog, 'água');
    expect(result.map((item) => item.id)).toEqual(['1']);
  });

  it('given_name_without_accent_when_search_then_matches_accented_name', () => {
    const result = filterProductsBySearch(catalog, 'agua');
    expect(result.map((item) => item.id)).toEqual(['1']);
  });

  it('given_partial_product_name_when_search_then_match', () => {
    const result = filterProductsBySearch(catalog, 'crystal');
    expect(result.map((item) => item.id)).toEqual(['1']);
  });

  it('given_empty_query_when_search_then_all_products', () => {
    expect(filterProductsBySearch(catalog, '   ')).toHaveLength(3);
  });

  it('given_no_match_when_search_then_empty', () => {
    expect(filterProductsBySearch(catalog, 'xyz-missing')).toEqual([]);
  });

  it('formats_product_option_as_sku_dash_name', () => {
    expect(formatProductOption(catalog[0]!)).toBe('SEED-003 — Água Crystal 500ml');
  });
});

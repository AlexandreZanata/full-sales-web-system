import { describe, expect, it } from 'vitest';

import type { PortalCategory, PortalProduct } from '@/lib/api/types';
import {
  filterProductsBySearch,
  resolveActiveCategorySlug,
  resolveDefaultCategorySlug,
  shouldRedirectToDefaultCategory,
} from '@/lib/catalog/catalogSearch';

const categories: PortalCategory[] = [
  {
    id: 'cat-2',
    name: 'Beverages',
    slug: 'beverages',
    sortOrder: 2,
    active: true,
  },
  {
    id: 'cat-1',
    name: 'Snacks',
    slug: 'snacks',
    sortOrder: 1,
    active: true,
  },
  {
    id: 'cat-3',
    name: 'Archived',
    slug: 'archived',
    sortOrder: 3,
    active: false,
  },
];

const products: PortalProduct[] = [
  {
    id: 'prod-1',
    name: 'Cola Zero',
    sku: 'BEV-001',
    priceAmount: 500,
    priceCurrency: 'BRL',
  },
  {
    id: 'prod-2',
    name: 'Potato Chips',
    sku: 'SNK-010',
    priceAmount: 800,
    priceCurrency: 'BRL',
  },
];

describe('catalogSearch — Phase 46 contract', () => {
  it('given_no_category_param_when_resolve_default_then_first_active_sort_order_slug', () => {
    expect(resolveDefaultCategorySlug(categories)).toBe('snacks');
  });

  it('given_unknown_slug_when_resolve_active_then_undefined', () => {
    expect(resolveActiveCategorySlug('missing', categories)).toBeUndefined();
  });

  it('given_valid_slug_when_resolve_active_then_matching_slug', () => {
    expect(resolveActiveCategorySlug('beverages', categories)).toBe('beverages');
  });

  it('given_no_category_param_when_should_redirect_then_true_with_categories', () => {
    expect(shouldRedirectToDefaultCategory(undefined, categories)).toBe(true);
  });

  it('given_category_param_when_should_redirect_then_false', () => {
    expect(shouldRedirectToDefaultCategory('snacks', categories)).toBe(false);
  });

  it('given_search_term_when_filter_products_then_matches_name_or_sku_within_category', () => {
    expect(filterProductsBySearch(products, 'cola')).toEqual([products[0]]);
    expect(filterProductsBySearch(products, 'snk-010')).toEqual([products[1]]);
    expect(filterProductsBySearch(products, '  ')).toEqual(products);
  });
});

/**
 * Contract: Phase 32 create/edit product form validation.
 */
import { describe, expect, it } from 'vitest';

import {
  hasFormErrors,
  toCreateProductPayload,
  validateCreateProductForm,
  validateEditProductForm,
} from '@/lib/products/validation';

describe('validateCreateProductForm — Phase 32 contract', () => {
  it('given_valid_form_when_validate_then_no_errors', () => {
    const errors = validateCreateProductForm({
      name: 'Widget',
      sku: 'SKU-001',
      price: '10,00',
      priceCurrency: 'BRL',
      categoryId: '',
      description: '',
    });
    expect(hasFormErrors(errors)).toBe(false);
  });

  it('given_empty_sku_when_validate_then_sku_required', () => {
    const errors = validateCreateProductForm({
      name: 'Widget',
      sku: '',
      price: '10,00',
      priceCurrency: 'BRL',
      categoryId: '',
      description: '',
    });
    expect(errors.sku).toBe('forms.validation.skuRequired');
  });

  it('given_description_over_limit_when_validate_then_description_error', () => {
    const errors = validateCreateProductForm({
      name: 'Widget',
      sku: 'SKU-001',
      price: '10,00',
      priceCurrency: 'BRL',
      categoryId: '',
      description: 'x'.repeat(2001),
    });
    expect(errors.description).toBe('forms.validation.descriptionMax');
  });
});

describe('toCreateProductPayload — API contract', () => {
  it('given_valid_form_when_to_payload_then_price_in_centavos', () => {
    const payload = toCreateProductPayload({
      name: 'Widget',
      sku: 'SKU-001',
      price: '10,50',
      priceCurrency: 'BRL',
      categoryId: 'cat-1',
      description: 'Cold drink for summer.',
    });
    expect(payload.priceAmount).toBe(1050);
    expect(payload.categoryId).toBe('cat-1');
  });

  it('given_no_category_when_to_payload_then_omits_category_id', () => {
    const payload = toCreateProductPayload({
      name: 'Widget',
      sku: 'SKU-001',
      price: '10,50',
      priceCurrency: 'BRL',
      categoryId: '',
      description: '',
    });
    expect(payload.categoryId).toBeUndefined();
  });
});

describe('validateEditProductForm — Phase 32 contract', () => {
  it('given_invalid_price_when_validate_then_price_error', () => {
    const errors = validateEditProductForm({
      name: 'Widget',
      price: '',
      priceCurrency: 'BRL',
      unitOfMeasure: 'Unit',
      categoryId: '',
      description: '',
    });
    expect(errors.price).toBe('forms.validation.priceInvalid');
  });
});

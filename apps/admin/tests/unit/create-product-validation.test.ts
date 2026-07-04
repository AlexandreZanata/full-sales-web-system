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
    });
    expect(hasFormErrors(errors)).toBe(false);
  });

  it('given_empty_sku_when_validate_then_sku_required', () => {
    const errors = validateCreateProductForm({
      name: 'Widget',
      sku: '',
      price: '10,00',
      priceCurrency: 'BRL',
    });
    expect(errors.sku).toBe('SKU is required');
  });
});

describe('toCreateProductPayload — API contract', () => {
  it('given_valid_form_when_to_payload_then_price_in_centavos', () => {
    const payload = toCreateProductPayload({
      name: 'Widget',
      sku: 'SKU-001',
      price: '10,50',
      priceCurrency: 'BRL',
    });
    expect(payload.priceAmount).toBe(1050);
  });
});

describe('validateEditProductForm — Phase 32 contract', () => {
  it('given_invalid_price_when_validate_then_price_error', () => {
    const errors = validateEditProductForm({
      name: 'Widget',
      price: '',
      priceCurrency: 'BRL',
    });
    expect(errors.price).toBe('Enter a valid price');
  });
});

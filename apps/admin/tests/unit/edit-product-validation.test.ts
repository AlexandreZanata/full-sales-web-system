/**
 * Contract: edit product form validation — unitOfMeasure parity with API PATCH body.
 */
import { describe, expect, it } from 'vitest';

import { toUpdateProductPayload, validateEditProductForm } from '@/lib/products/validation';

describe('edit product validation — Phase 43 contract', () => {
  it('given_empty_unit_of_measure_when_validate_then_unit_of_measure_required', () => {
    const errors = validateEditProductForm({
      name: 'Widget',
      price: '10.00',
      priceCurrency: 'BRL',
      unitOfMeasure: '   ',
    });

    expect(errors.unitOfMeasure).toBe('forms.validation.unitOfMeasureRequired');
  });

  it('given_valid_values_when_to_update_payload_then_includes_unit_without_legacy_category', () => {
    const payload = toUpdateProductPayload({
      name: '  Widget  ',
      price: '12,50',
      priceCurrency: 'BRL',
      unitOfMeasure: ' Box ',
    });

    expect(payload).toEqual({
      name: 'Widget',
      priceAmount: 1250,
      priceCurrency: 'BRL',
      unitOfMeasure: 'Box',
    });
  });
});

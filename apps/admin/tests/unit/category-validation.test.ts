/**
 * Contract: Phase 44 category form validation.
 */
import { describe, expect, it } from 'vitest';

import {
  hasCategoryFormErrors,
  toCreateCategoryPayload,
  validateCategoryForm,
} from '@/lib/categories/validation';

describe('validateCategoryForm — Phase 44 contract', () => {
  it('given_empty_name_when_validate_then_name_required', () => {
    const errors = validateCategoryForm({
      name: '   ',
      description: '',
      sortOrder: '',
      active: true,
    });

    expect(errors.name).toBe('forms.validation.nameRequired');
  });

  it('given_invalid_sort_order_when_validate_then_sort_order_error', () => {
    const errors = validateCategoryForm({
      name: 'Bebidas',
      description: '',
      sortOrder: '-1',
      active: true,
    });

    expect(errors.sortOrder).toBe('categories.validation.sortOrderInvalid');
  });

  it('given_valid_form_when_to_create_payload_then_trims_name', () => {
    const payload = toCreateCategoryPayload({
      name: '  Bebidas  ',
      description: '  Cold drinks  ',
      sortOrder: '3',
      active: true,
    });

    expect(payload).toEqual({
      name: 'Bebidas',
      description: 'Cold drinks',
      sortOrder: 3,
      active: true,
    });
    expect(
      hasCategoryFormErrors(
        validateCategoryForm({
          name: 'Bebidas',
          description: '',
          sortOrder: '3',
          active: true,
        }),
      ),
    ).toBe(false);
  });
});

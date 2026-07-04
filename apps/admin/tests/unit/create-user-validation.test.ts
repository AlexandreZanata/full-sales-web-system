/**
 * Contract: Phase 30 create user form — required fields + CommerceContact commerceId.
 */
import { describe, expect, it } from 'vitest';

import {
  hasCreateUserFormErrors,
  validateCreateUserForm,
  type CreateUserFormValues,
} from '@/lib/users/validation';

const validBase: CreateUserFormValues = {
  name: 'Jane Driver',
  email: 'driver@test.com',
  password: 'secret123',
  role: 'Driver',
  commerceId: '',
};

describe('validateCreateUserForm — Phase 30 contract', () => {
  it('given_valid_driver_when_validate_then_no_errors', () => {
    expect(hasCreateUserFormErrors(validateCreateUserForm(validBase))).toBe(false);
  });

  it('given_empty_name_when_validate_then_name_required', () => {
    const errors = validateCreateUserForm({ ...validBase, name: '  ' });
    expect(errors.name).toBe('forms.validation.nameRequired');
  });

  it('given_invalid_email_when_validate_then_email_error', () => {
    const errors = validateCreateUserForm({ ...validBase, email: 'not-an-email' });
    expect(errors.email).toBe('forms.validation.emailInvalid');
  });

  it('given_short_password_when_validate_then_password_error', () => {
    const errors = validateCreateUserForm({ ...validBase, password: 'short' });
    expect(errors.password).toBe('forms.validation.passwordMinLength');
  });

  it('given_commerce_contact_without_commerce_when_validate_then_commerce_required', () => {
    const errors = validateCreateUserForm({
      ...validBase,
      role: 'CommerceContact',
      commerceId: '',
    });
    expect(errors.commerceId).toBe('forms.validation.commerceContactRequired');
  });

  it('given_commerce_contact_with_commerce_when_validate_then_no_errors', () => {
    const errors = validateCreateUserForm({
      ...validBase,
      role: 'CommerceContact',
      commerceId: '550e8400-e29b-41d4-a716-446655440000',
    });
    expect(hasCreateUserFormErrors(errors)).toBe(false);
  });
});

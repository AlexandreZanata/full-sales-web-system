/**
 * Contract: Phase 31 create commerce form — CNPJ, legal name, address, contact validation.
 */
import { describe, expect, it } from 'vitest';

import {
  hasFormErrors,
  toCreateCommercePayload,
  validateCreateCommerceForm,
  validateAddressForm,
  type CreateCommerceFormValues,
  type AddressFormValues,
} from '@/lib/commerces/validation';

const validCommerce: CreateCommerceFormValues = {
  cnpj: '11.444.777/0001-61',
  legalName: 'Acme Ltda',
  tradeName: 'Acme Store',
  street: 'Rua A',
  number: '100',
  district: 'Centro',
  city: 'São Paulo',
  state: 'SP',
  postalCode: '01310100',
  contactPhone: '11999999999',
  contactEmail: 'contact@acme.test',
};

describe('validateCreateCommerceForm — Phase 31 contract', () => {
  it('given_valid_form_when_validate_then_no_errors', () => {
    expect(hasFormErrors(validateCreateCommerceForm(validCommerce))).toBe(false);
  });

  it('given_invalid_cnpj_when_validate_then_cnpj_error', () => {
    const errors = validateCreateCommerceForm({ ...validCommerce, cnpj: '11.111.111/1111-11' });
    expect(errors.cnpj).toBe('Enter a valid CNPJ');
  });

  it('given_empty_legal_name_when_validate_then_required', () => {
    const errors = validateCreateCommerceForm({ ...validCommerce, legalName: '  ' });
    expect(errors.legalName).toBe('Legal name is required');
  });

  it('given_invalid_contact_email_when_validate_then_email_error', () => {
    const errors = validateCreateCommerceForm({ ...validCommerce, contactEmail: 'bad' });
    expect(errors.contactEmail).toBe('Enter a valid email address');
  });
});

describe('toCreateCommercePayload — API contract', () => {
  it('given_valid_form_when_to_payload_then_strips_cnpj_and_uppercases_state', () => {
    const payload = toCreateCommercePayload(validCommerce);
    expect(payload.cnpj).toBe('11444777000161');
    expect(payload.address.state).toBe('SP');
    expect(payload.contact.email).toBe('contact@acme.test');
  });
});

const validAddress: AddressFormValues = {
  addressType: 'Delivery',
  street: 'Rua B',
  number: '50',
  district: '',
  city: 'São Paulo',
  state: 'SP',
  postalCode: '01310100',
  isPrimary: true,
};

describe('validateAddressForm — Phase 31 contract', () => {
  it('given_valid_delivery_address_when_validate_then_no_errors', () => {
    expect(hasFormErrors(validateAddressForm(validAddress))).toBe(false);
  });

  it('given_missing_type_when_validate_then_type_required', () => {
    const errors = validateAddressForm({ ...validAddress, addressType: '' });
    expect(errors.addressType).toBe('Select an address type');
  });
});

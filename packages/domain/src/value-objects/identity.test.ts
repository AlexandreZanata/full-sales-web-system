import { describe, expect, it } from 'vitest';

import {
  InvalidEmailError,
  InvalidFullNameError,
  InvalidSkuError,
  InvalidUuidError,
} from '../errors/domain-error.js';
import { Email } from './email.js';
import { FullName } from './full-name.js';
import { parseSaleId } from './ids.js';
import { Sku } from './sku.js';

describe('Email', () => {
  it('given_valid_email_when_parse_then_lowercased', () => {
    expect(Email.parse('User@Example.COM').toString()).toBe('user@example.com');
  });

  it('given_invalid_email_when_parse_then_invalid_email', () => {
    expect(() => Email.parse('not-an-email')).toThrow(InvalidEmailError);
  });
});

describe('FullName', () => {
  it('given_two_part_name_when_parse_then_ok', () => {
    expect(FullName.parse('Jane Doe').toString()).toBe('Jane Doe');
  });

  it('given_single_name_when_parse_then_invalid_full_name', () => {
    expect(() => FullName.parse('Madonna')).toThrow(InvalidFullNameError);
  });
});

describe('Sku', () => {
  it('given_valid_sku_when_parse_then_ok', () => {
    expect(Sku.parse('SKU-001').toString()).toBe('SKU-001');
  });

  it('given_empty_sku_when_parse_then_invalid_sku', () => {
    expect(() => Sku.parse('   ')).toThrow(InvalidSkuError);
  });
});

describe('SaleId', () => {
  it('given_invalid_uuid_when_parse_then_invalid_uuid', () => {
    expect(() => parseSaleId('not-a-uuid')).toThrow(InvalidUuidError);
  });

  it('given_valid_uuid_when_parse_then_ok', () => {
    const id = '550e8400-e29b-41d4-a716-446655440000';
    expect(parseSaleId(id)).toBe(id);
  });
});

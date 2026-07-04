import { describe, expect, it } from 'vitest';

import { isFieldRole } from '@/lib/auth/jwt';
import { saleActionErrorMessage } from '@/lib/sales/constants';

describe('isFieldRole', () => {
  it('accepts Driver and Seller', () => {
    expect(isFieldRole('Driver')).toBe(true);
    expect(isFieldRole('Seller')).toBe(true);
  });

  it('rejects Admin', () => {
    expect(isFieldRole('Admin')).toBe(false);
  });
});

describe('saleActionErrorMessage', () => {
  it('returns pt-BR message for INSUFFICIENT_STOCK', () => {
    expect(saleActionErrorMessage('INSUFFICIENT_STOCK')).toContain('Estoque');
  });
});

import { describe, expect, it } from 'vitest';

import { isCommerceContactRole } from '@/lib/auth/jwt';

describe('isCommerceContactRole', () => {
  it('accepts CommerceContact role', () => {
    expect(isCommerceContactRole('CommerceContact')).toBe(true);
  });

  it('rejects Admin role', () => {
    expect(isCommerceContactRole('Admin')).toBe(false);
  });
});

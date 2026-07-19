import { afterEach, describe, expect, it } from 'vitest';

import {
  clearSellerAttribution,
  readSellerAttribution,
  resetSellerAttributionCacheForTests,
  resolveContactPhone,
  writeSellerAttribution,
} from '@/lib/seller/attribution';

afterEach(() => {
  clearSellerAttribution();
  resetSellerAttributionCacheForTests();
});

describe('seller attribution', () => {
  it('persists_and_clears_session_attribution', () => {
    writeSellerAttribution({
      publicCode: 'maria',
      displayName: 'Maria',
      contactPhone: '11999998888',
    });
    expect(readSellerAttribution()?.publicCode).toBe('maria');
    clearSellerAttribution();
    expect(readSellerAttribution()).toBeNull();
  });

  it('getSnapshot_returns_stable_reference_until_write', () => {
    writeSellerAttribution({ publicCode: 'a', displayName: 'A' });
    const first = readSellerAttribution();
    const second = readSellerAttribution();
    expect(first).toBe(second);
  });

  it('resolves_phone_precedence_seller_then_tenant', () => {
    expect(
      resolveContactPhone(
        { publicCode: 'a', displayName: 'A', contactPhone: '11911112222' },
        '11999998888',
      ),
    ).toBe('11911112222');
    expect(
      resolveContactPhone({ publicCode: 'a', displayName: 'A' }, '11999998888'),
    ).toBe('11999998888');
    expect(resolveContactPhone(null, '11999998888')).toBe('11999998888');
    expect(resolveContactPhone(null, undefined)).toBeUndefined();
  });
});

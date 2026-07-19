import { afterEach, describe, expect, it, vi } from 'vitest';

import { ApiError } from '@/lib/api/client';
import * as publicSellers from '@/lib/api/publicSellers';
import {
  clearSellerAttribution,
  readSellerAttribution,
  resetSellerAttributionCacheForTests,
  resolveContactPhone,
  writeSellerAttribution,
} from '@/lib/seller/attribution';
import { refreshSellerAttribution } from '@/lib/seller/refreshSellerAttribution';

afterEach(() => {
  clearSellerAttribution();
  resetSellerAttributionCacheForTests();
  vi.restoreAllMocks();
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

  it('refresh_updates_stale_display_name_from_api', async () => {
    writeSellerAttribution({ publicCode: 'dev', displayName: 'Dev Seller' });
    vi.spyOn(publicSellers, 'fetchPublicSeller').mockResolvedValue({
      publicCode: 'dev',
      displayName: 'Novo Nome',
      contactPhone: '11900001111',
    });
    await refreshSellerAttribution('dev');
    expect(readSellerAttribution()?.displayName).toBe('Novo Nome');
    expect(readSellerAttribution()?.contactPhone).toBe('11900001111');
  });

  it('refresh_clears_attribution_when_seller_not_found', async () => {
    writeSellerAttribution({ publicCode: 'gone', displayName: 'Gone' });
    vi.spyOn(publicSellers, 'fetchPublicSeller').mockRejectedValue(
      new ApiError(404, { error: { code: 'NOT_FOUND', message: 'not found' } }),
    );
    await refreshSellerAttribution('gone');
    expect(readSellerAttribution()).toBeNull();
  });
});

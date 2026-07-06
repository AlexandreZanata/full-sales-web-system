/**
 * Contract: docs/API-CONTRACT.md — GET /v1/commerces cursor envelope.
 */
import { afterEach, describe, expect, it, vi } from 'vitest';

import { fetchCommerces, fetchCommerceAddresses } from '@/lib/api/commerces';

describe('commerces API — docs/API-CONTRACT.md', () => {
  afterEach(() => {
    vi.unstubAllGlobals();
  });

  it('fetchCommerces passes filter[active] query param', async () => {
    const fetchMock = vi.fn().mockResolvedValue({
      ok: true,
      status: 200,
      json: async () => ({
        data: [],
        pagination: { next_cursor: null, has_more: false, limit: 20 },
      }),
    });
    vi.stubGlobal('fetch', fetchMock);

    await fetchCommerces({ limit: 20, active: 'true' });

    expect(fetchMock).toHaveBeenCalledWith(
      '/v1/commerces?limit=20&filter%5Bactive%5D=true',
      expect.any(Object),
    );
  });

  it('fetchCommerceAddresses calls cursor list endpoint', async () => {
    const fetchMock = vi.fn().mockResolvedValue({
      ok: true,
      status: 200,
      json: async () => ({
        data: [{ id: 'addr-1', addressType: 'Billing', street: 'Rua A', number: '1', city: 'SP', state: 'SP', postalCode: '01310100', isPrimary: true }],
        pagination: { next_cursor: null, has_more: false, limit: 100 },
      }),
    });
    vi.stubGlobal('fetch', fetchMock);

    const result = await fetchCommerceAddresses('commerce-1');

    expect(fetchMock).toHaveBeenCalledWith(
      '/v1/commerces/commerce-1/addresses?limit=100',
      expect.any(Object),
    );
    expect(result).toHaveLength(1);
  });
});

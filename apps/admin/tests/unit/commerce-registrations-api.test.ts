/**
 * Contract: commerce registration API query params (Phase 69).
 */
import { afterEach, describe, expect, it, vi } from 'vitest';

import { fetchCommerceRegistrations } from '@/lib/api/commerceRegistrations';

describe('commerce registrations API', () => {
  afterEach(() => {
    vi.unstubAllGlobals();
  });

  it('fetchCommerceRegistrations passes filter[status] query param', async () => {
    const fetchMock = vi.fn().mockResolvedValue({
      ok: true,
      status: 200,
      json: async () => ({
        data: [],
        pagination: { next_cursor: null, has_more: false, limit: 20 },
      }),
    });
    vi.stubGlobal('fetch', fetchMock);

    await fetchCommerceRegistrations({ limit: 20, status: 'PendingReview' });

    expect(fetchMock).toHaveBeenCalledWith(
      '/v1/commerces/registrations?limit=20&filter%5Bstatus%5D=PendingReview',
      expect.any(Object),
    );
  });
});

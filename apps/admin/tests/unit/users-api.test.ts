/**
 * Contract: docs/API-CONTRACT.md — GET /v1/users cursor envelope.
 */
import { afterEach, describe, expect, it, vi } from 'vitest';

import { fetchUsers } from '@/lib/api/users';

describe('users API — docs/API-CONTRACT.md', () => {
  afterEach(() => {
    vi.unstubAllGlobals();
  });

  it('fetchUsers passes filter[role] query param', async () => {
    const fetchMock = vi.fn().mockResolvedValue({
      ok: true,
      status: 200,
      json: async () => ({
        data: [],
        pagination: { next_cursor: null, has_more: false, limit: 20 },
      }),
    });
    vi.stubGlobal('fetch', fetchMock);

    await fetchUsers({ limit: 20, role: 'Driver' });

    expect(fetchMock).toHaveBeenCalledWith(
      '/v1/users?limit=20&filter%5Brole%5D=Driver',
      expect.any(Object),
    );
  });
});

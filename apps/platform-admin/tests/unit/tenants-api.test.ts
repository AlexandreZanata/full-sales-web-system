/**
 * Contract: docs/API-CONTRACT.md — GET /v1/platform/tenants query filters.
 */
import { afterEach, describe, expect, it, vi } from 'vitest';

import { fetchTenants } from '@/lib/api/tenants';

describe('tenants API — Phase 11 contract', () => {
  afterEach(() => {
    vi.unstubAllGlobals();
  });

  it('builds_filter_status_query_param', async () => {
    const fetchMock = vi.fn().mockResolvedValue({
      ok: true,
      status: 200,
      json: async () => ({ data: [], meta: {} }),
    });
    vi.stubGlobal('fetch', fetchMock);

    await fetchTenants({ limit: 20, status: 'Suspended' });

    expect(fetchMock).toHaveBeenCalledWith(
      expect.stringContaining('filter%5Bstatus%5D=Suspended'),
      expect.anything(),
    );
  });
});

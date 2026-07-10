/**
 * Contract: docs/API-CONTRACT.md — billing tenant API query strings.
 */
import { afterEach, describe, expect, it, vi } from 'vitest';

import { fetchInvoices, fetchSubscription } from '@/lib/api/billing';

describe('billing API — Phase 12 contract', () => {
  afterEach(() => {
    vi.unstubAllGlobals();
  });

  it('fetchSubscription_calls_billing_subscription', async () => {
    const fetchMock = vi.fn().mockResolvedValue({
      ok: true,
      status: 200,
      json: async () => ({ plan: {}, status: 'Active', tenantStatus: 'Active' }),
    });
    vi.stubGlobal('fetch', fetchMock);

    await fetchSubscription();

    expect(fetchMock).toHaveBeenCalledWith(
      '/v1/billing/subscription',
      expect.objectContaining({ headers: expect.objectContaining({ Accept: 'application/json' }) }),
    );
  });

  it('fetchInvoices_builds_limit_query', async () => {
    const fetchMock = vi.fn().mockResolvedValue({
      ok: true,
      status: 200,
      json: async () => ({ data: [] }),
    });
    vi.stubGlobal('fetch', fetchMock);

    await fetchInvoices({ limit: 25 });

    expect(fetchMock).toHaveBeenCalledWith(expect.stringContaining('limit=25'), expect.anything());
  });
});

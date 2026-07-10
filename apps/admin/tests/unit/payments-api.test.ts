/**
 * Contract: docs/API-CONTRACT.md — settings payments endpoints.
 */
import { afterEach, describe, expect, it, vi } from 'vitest';

import { fetchPaymentSettings } from '@/lib/api/payments';

describe('payments API — Phase 12 contract', () => {
  afterEach(() => {
    vi.unstubAllGlobals();
  });

  it('fetchPaymentSettings_calls_settings_payments', async () => {
    const fetchMock = vi.fn().mockResolvedValue({
      ok: true,
      status: 200,
      json: async () => ({
        enabled: false,
        methods: { pix: true, credit: false, boleto: false },
        autoCapture: true,
        asaas: { connected: false },
      }),
    });
    vi.stubGlobal('fetch', fetchMock);

    await fetchPaymentSettings();

    expect(fetchMock).toHaveBeenCalledWith('/v1/settings/payments', expect.anything());
  });
});

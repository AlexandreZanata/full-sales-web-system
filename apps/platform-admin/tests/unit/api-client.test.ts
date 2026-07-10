/**
 * Contract: docs/API-CONTRACT.md — platform refresh at /v1/platform/auth/refresh.
 */
import { afterEach, describe, expect, it, vi } from 'vitest';

import { ApiError, apiFetch, getApiBaseUrl, parseApiErrorBody } from '@/lib/api/client';

describe('platform API client — Phase 11 contract', () => {
  afterEach(() => {
    vi.unstubAllGlobals();
  });

  it('builds URLs under /v1', async () => {
    expect(getApiBaseUrl()).toBe('/v1');

    const fetchMock = vi.fn().mockResolvedValue({
      ok: true,
      status: 200,
      json: async () => ({ data: [] }),
    });
    vi.stubGlobal('fetch', fetchMock);

    await apiFetch('/platform/tenants');

    expect(fetchMock).toHaveBeenCalledWith(
      '/v1/platform/tenants',
      expect.objectContaining({
        headers: expect.objectContaining({ Accept: 'application/json' }) as Record<string, string>,
      }),
    );
  });

  it('parses contract error shape on 401', async () => {
    vi.stubGlobal(
      'fetch',
      vi.fn().mockResolvedValue({
        ok: false,
        status: 401,
        json: async () => ({
          error: { code: 'UNAUTHORIZED', message: 'Missing or invalid token' },
        }),
      }),
    );

    await expect(apiFetch('/platform/tenants')).rejects.toMatchObject({
      status: 401,
      code: 'UNAUTHORIZED',
    } satisfies Partial<ApiError>);
  });

  it('parseApiErrorBody falls back when response is not JSON', async () => {
    const response = {
      ok: false,
      status: 500,
      statusText: 'Internal Server Error',
      json: async () => {
        throw new Error('not json');
      },
    } as unknown as Response;

    const parsed = await parseApiErrorBody(response);
    expect(parsed.error.code).toBe('HTTP_ERROR');
  });
});

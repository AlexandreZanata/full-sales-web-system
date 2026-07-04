/**
 * Contract: docs/API-CONTRACT.md — error shape { error: { code, message, correlationId } }.
 */
import { afterEach, describe, expect, it, vi } from 'vitest';

import { ApiError, apiFetch, getApiBaseUrl, parseApiErrorBody } from '@/lib/api/client';

describe('API client — docs/API-CONTRACT.md', () => {
  afterEach(() => {
    vi.unstubAllGlobals();
  });

  it('builds URLs under /v1', async () => {
    expect(getApiBaseUrl()).toBe('/v1');

    const fetchMock = vi.fn().mockResolvedValue({
      ok: true,
      status: 200,
      json: async () => ({ items: [] }),
    });
    vi.stubGlobal('fetch', fetchMock);

    await apiFetch('/users');

    expect(fetchMock).toHaveBeenCalledWith(
      '/v1/users',
      expect.objectContaining({
        headers: expect.objectContaining({ Accept: 'application/json' }) as Record<string, string>,
      }),
    );
  });

  it('parses contract error shape on 401 unauthorized', async () => {
    const body = {
      error: {
        code: 'UNAUTHORIZED',
        message: 'Missing or invalid token',
        correlationId: '550e8400-e29b-41d4-a716-446655440000',
      },
    };

    vi.stubGlobal(
      'fetch',
      vi.fn().mockResolvedValue({
        ok: false,
        status: 401,
        json: async () => body,
      }),
    );

    await expect(apiFetch('/users')).rejects.toMatchObject({
      status: 401,
      code: 'UNAUTHORIZED',
      message: 'Missing or invalid token',
      correlationId: '550e8400-e29b-41d4-a716-446655440000',
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
    expect(parsed.error.message).toBe('Internal Server Error');
  });
});

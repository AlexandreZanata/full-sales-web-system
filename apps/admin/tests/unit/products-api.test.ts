/**
 * Contract: docs/API-CONTRACT.md — GET /v1/products/{id}/images returns cursor envelope.
 */
import { afterEach, describe, expect, it, vi } from 'vitest';

import { fetchProductImages, fetchProducts } from '@/lib/api/products';

describe('products API — docs/API-CONTRACT.md', () => {
  afterEach(() => {
    vi.unstubAllGlobals();
  });

  it('fetchProductImages calls GET /products/{id}/images', async () => {
    const fetchMock = vi.fn().mockResolvedValue({
      ok: true,
      status: 200,
      json: async () => ({
        data: [
          {
            id: 'img-1',
            fileId: 'file-1',
            sortOrder: 0,
            isPrimary: true,
          },
        ],
        pagination: { next_cursor: null, has_more: false, limit: 50 },
      }),
    });
    vi.stubGlobal('fetch', fetchMock);

    const result = await fetchProductImages('prod-1');

    expect(fetchMock).toHaveBeenCalledWith(
      '/v1/products/prod-1/images?limit=50',
      expect.objectContaining({
        headers: expect.objectContaining({ Accept: 'application/json' }) as Record<string, string>,
      }),
    );
    expect(result.data).toHaveLength(1);
    expect(result.data[0]?.isPrimary).toBe(true);
  });

  it('fetchProducts passes filter[active] query param', async () => {
    const fetchMock = vi.fn().mockResolvedValue({
      ok: true,
      status: 200,
      json: async () => ({
        data: [],
        pagination: { next_cursor: null, has_more: false, limit: 20 },
      }),
    });
    vi.stubGlobal('fetch', fetchMock);

    await fetchProducts({ limit: 20, active: 'false' });

    expect(fetchMock).toHaveBeenCalledWith(
      '/v1/products?limit=20&filter%5Bactive%5D=false',
      expect.any(Object),
    );
  });
});

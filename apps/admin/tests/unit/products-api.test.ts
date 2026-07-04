/**
 * Contract: docs/API-CONTRACT.md — GET /v1/products/{id}/images returns { items: ProductImage[] }.
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
        items: [
          {
            id: 'img-1',
            fileId: 'file-1',
            sortOrder: 0,
            isPrimary: true,
          },
        ],
      }),
    });
    vi.stubGlobal('fetch', fetchMock);

    const result = await fetchProductImages('prod-1');

    expect(fetchMock).toHaveBeenCalledWith(
      '/v1/products/prod-1/images',
      expect.objectContaining({
        headers: expect.objectContaining({ Accept: 'application/json' }) as Record<string, string>,
      }),
    );
    expect(result.items).toHaveLength(1);
    expect(result.items[0]?.isPrimary).toBe(true);
  });

  it('fetchProducts passes active filter query param', async () => {
    const fetchMock = vi.fn().mockResolvedValue({
      ok: true,
      status: 200,
      json: async () => ({ items: [], page: 1, pageSize: 20, total: 0 }),
    });
    vi.stubGlobal('fetch', fetchMock);

    await fetchProducts({ page: 1, pageSize: 20, active: 'false' });

    expect(fetchMock).toHaveBeenCalledWith(
      '/v1/products?page=1&pageSize=20&active=false',
      expect.any(Object),
    );
  });
});

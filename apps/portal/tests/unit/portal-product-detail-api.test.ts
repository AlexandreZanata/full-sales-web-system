/**
 * Contract: docs/API-CONTRACT.md — Phase 48 portal product detail endpoint.
 */
import { afterEach, describe, expect, it, vi } from 'vitest';

import { fetchPortalProductById } from '@/lib/api/portal';

const MOCK_DETAIL = {
  id: 'prod-1',
  name: 'Seed Widget',
  sku: 'SKU-001',
  priceAmount: 1500,
  priceCurrency: 'BRL',
  unitOfMeasure: 'UN',
  primaryImageUrl: 'https://cdn.example/primary.jpg',
  imageUrls: ['https://cdn.example/gallery-1.jpg', 'https://cdn.example/gallery-2.jpg'],
  description: 'A seeded product for tests.',
};

describe('portal product detail API — docs/API-CONTRACT.md Phase 48', () => {
  afterEach(() => {
    vi.unstubAllGlobals();
    vi.resetModules();
  });

  it('fetchPortalProductById calls GET /public/products/{id} without session', async () => {
    const fetchMock = vi.fn().mockResolvedValue({
      ok: true,
      status: 200,
      json: async () => MOCK_DETAIL,
    });
    vi.stubGlobal('fetch', fetchMock);

    const result = await fetchPortalProductById('prod-1');

    expect(fetchMock).toHaveBeenCalledWith(
      '/v1/public/products/prod-1',
      expect.objectContaining({
        headers: expect.not.objectContaining({ Authorization: expect.any(String) as string }),
      }),
    );
    expect(result?.unitOfMeasure).toBe('UN');
    expect(result?.imageUrls).toHaveLength(2);
  });

  it('fetchPortalProductById returns null on 404', async () => {
    const fetchMock = vi.fn().mockResolvedValue({
      ok: false,
      status: 404,
      statusText: 'Not Found',
      json: async () => ({ error: { code: 'NOT_FOUND', message: 'Product not found' } }),
    });
    vi.stubGlobal('fetch', fetchMock);

    const result = await fetchPortalProductById('missing');

    expect(result).toBeNull();
  });
});

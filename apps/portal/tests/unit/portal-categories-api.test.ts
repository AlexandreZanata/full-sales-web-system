/**
 * Contract: docs/API-CONTRACT.md — Phase 68E portal/public category cursor endpoints.
 */
import { afterEach, describe, expect, it, vi } from 'vitest';

import { fetchPortalCategories, fetchPortalCategoryBySlug } from '@/lib/api/portal';

describe('portal categories API — cursor envelope', () => {
  afterEach(() => {
    vi.unstubAllGlobals();
    vi.resetModules();
  });

  it('fetchPortalCategories calls GET /public/categories with limit', async () => {
    const fetchMock = vi.fn().mockResolvedValue({
      ok: true,
      status: 200,
      json: async () => ({
        data: [{ id: 'cat-1', name: 'Bebidas', slug: 'bebidas', sortOrder: 0, active: true }],
        pagination: { next_cursor: null, has_more: false, limit: 100 },
      }),
    });
    vi.stubGlobal('fetch', fetchMock);

    const result = await fetchPortalCategories();

    expect(fetchMock).toHaveBeenCalledWith(
      '/v1/public/categories?limit=100',
      expect.objectContaining({
        headers: expect.not.objectContaining({ Authorization: expect.any(String) as string }),
      }),
    );
    expect(result).toHaveLength(1);
    expect(result[0]?.slug).toBe('bebidas');
  });

  it('fetchPortalCategoryBySlug calls GET /public/categories/{slug} with cursor params', async () => {
    const fetchMock = vi.fn().mockResolvedValue({
      ok: true,
      status: 200,
      json: async () => ({
        id: 'cat-1',
        name: 'Bebidas',
        slug: 'bebidas',
        sortOrder: 0,
        active: true,
        products: [],
        pagination: { next_cursor: null, has_more: false, limit: 50 },
      }),
    });
    vi.stubGlobal('fetch', fetchMock);

    await fetchPortalCategoryBySlug('bebidas', { limit: 50, cursor: 'cursor-1' });

    expect(fetchMock).toHaveBeenCalledWith(
      '/v1/public/categories/bebidas?limit=50&cursor=cursor-1',
      expect.any(Object),
    );
  });
});

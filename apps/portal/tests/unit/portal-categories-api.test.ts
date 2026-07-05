/**
 * Contract: docs/API-CONTRACT.md — Phase 43 portal/public category endpoints.
 */
import { afterEach, describe, expect, it, vi } from 'vitest';

import { fetchPortalCategories, fetchPortalCategoryBySlug } from '@/lib/api/portal';

describe('portal categories API — docs/API-CONTRACT.md Phase 43', () => {
  afterEach(() => {
    vi.unstubAllGlobals();
    vi.resetModules();
  });

  it('fetchPortalCategories calls GET /public/categories without session', async () => {
    const fetchMock = vi.fn().mockResolvedValue({
      ok: true,
      status: 200,
      json: async () => [
        { id: 'cat-1', name: 'Bebidas', slug: 'bebidas', sortOrder: 0, active: true },
      ],
    });
    vi.stubGlobal('fetch', fetchMock);

    const result = await fetchPortalCategories();

    expect(fetchMock).toHaveBeenCalledWith(
      '/v1/public/categories',
      expect.objectContaining({
        headers: expect.not.objectContaining({ Authorization: expect.any(String) as string }),
      }),
    );
    expect(result).toHaveLength(1);
    expect(result[0]?.slug).toBe('bebidas');
  });

  it('fetchPortalCategoryBySlug calls GET /public/categories/{slug}', async () => {
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
        page: 1,
        pageSize: 50,
        total: 0,
      }),
    });
    vi.stubGlobal('fetch', fetchMock);

    await fetchPortalCategoryBySlug('bebidas', { page: 1, pageSize: 50 });

    expect(fetchMock).toHaveBeenCalledWith(
      '/v1/public/categories/bebidas?page=1&pageSize=50',
      expect.any(Object),
    );
  });
});

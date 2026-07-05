/**
 * Contract: docs/API-CONTRACT.md — Phase 43 category endpoints.
 */
import { afterEach, describe, expect, it, vi } from 'vitest';

import {
  createCategory,
  deactivateCategory,
  fetchCategories,
  fetchCategory,
  reorderCategories,
  updateCategory,
  uploadCategoryImage,
} from '@/lib/api/categories';

describe('categories API — docs/API-CONTRACT.md Phase 43', () => {
  afterEach(() => {
    vi.unstubAllGlobals();
  });

  it('fetchCategories passes active filter query param', async () => {
    const fetchMock = vi.fn().mockResolvedValue({
      ok: true,
      status: 200,
      json: async () => ({ items: [], page: 1, pageSize: 20, total: 0 }),
    });
    vi.stubGlobal('fetch', fetchMock);

    await fetchCategories({ page: 1, pageSize: 20, active: 'true' });

    expect(fetchMock).toHaveBeenCalledWith(
      '/v1/categories?page=1&pageSize=20&active=true',
      expect.any(Object),
    );
  });

  it('fetchCategory calls GET /categories/{id}', async () => {
    const fetchMock = vi.fn().mockResolvedValue({
      ok: true,
      status: 200,
      json: async () => ({
        id: 'cat-1',
        name: 'Bebidas',
        slug: 'bebidas',
        sortOrder: 0,
        active: true,
      }),
    });
    vi.stubGlobal('fetch', fetchMock);

    await fetchCategory('cat-1');

    expect(fetchMock).toHaveBeenCalledWith('/v1/categories/cat-1', expect.any(Object));
  });

  it('createCategory posts name body to POST /categories', async () => {
    const fetchMock = vi.fn().mockResolvedValue({
      ok: true,
      status: 201,
      json: async () => ({
        id: 'cat-1',
        name: 'Bebidas',
        slug: 'bebidas',
        sortOrder: 0,
        active: true,
      }),
    });
    vi.stubGlobal('fetch', fetchMock);

    await createCategory({ name: 'Bebidas', active: true });

    expect(fetchMock).toHaveBeenCalledWith(
      '/v1/categories',
      expect.objectContaining({
        method: 'POST',
        body: JSON.stringify({ name: 'Bebidas', active: true }),
      }),
    );
  });

  it('updateCategory patches body to PATCH /categories/{id}', async () => {
    const fetchMock = vi.fn().mockResolvedValue({
      ok: true,
      status: 200,
      json: async () => ({
        id: 'cat-1',
        name: 'Drinks',
        slug: 'drinks',
        sortOrder: 0,
        active: true,
      }),
    });
    vi.stubGlobal('fetch', fetchMock);

    await updateCategory('cat-1', { name: 'Drinks' });

    expect(fetchMock).toHaveBeenCalledWith(
      '/v1/categories/cat-1',
      expect.objectContaining({
        method: 'PATCH',
        body: JSON.stringify({ name: 'Drinks' }),
      }),
    );
  });

  it('deactivateCategory calls DELETE /categories/{id}', async () => {
    const fetchMock = vi.fn().mockResolvedValue({
      ok: true,
      status: 204,
    });
    vi.stubGlobal('fetch', fetchMock);

    await deactivateCategory('cat-1');

    expect(fetchMock).toHaveBeenCalledWith(
      '/v1/categories/cat-1',
      expect.objectContaining({ method: 'DELETE' }),
    );
  });

  it('reorderCategories posts orderedIds to POST /categories/reorder', async () => {
    const fetchMock = vi.fn().mockResolvedValue({
      ok: true,
      status: 204,
    });
    vi.stubGlobal('fetch', fetchMock);

    await reorderCategories(['cat-2', 'cat-1']);

    expect(fetchMock).toHaveBeenCalledWith(
      '/v1/categories/reorder',
      expect.objectContaining({
        method: 'POST',
        body: JSON.stringify({ orderedIds: ['cat-2', 'cat-1'] }),
      }),
    );
  });

  it('uploadCategoryImage puts fileId to PUT /categories/{id}/image', async () => {
    const fetchMock = vi.fn().mockResolvedValue({
      ok: true,
      status: 200,
      json: async () => ({
        id: 'cat-1',
        name: 'Bebidas',
        slug: 'bebidas',
        sortOrder: 0,
        active: true,
        imageFileId: 'file-1',
      }),
    });
    vi.stubGlobal('fetch', fetchMock);

    await uploadCategoryImage('cat-1', 'file-1');

    expect(fetchMock).toHaveBeenCalledWith(
      '/v1/categories/cat-1/image',
      expect.objectContaining({
        method: 'PUT',
        body: JSON.stringify({ fileId: 'file-1' }),
      }),
    );
  });
});

import { beforeEach, describe, expect, it, vi } from 'vitest';

import { fetchPortalPopularProducts } from '@/lib/api/portalPopular';

const apiFetch = vi.fn();
const fetchPortalProducts = vi.fn();

vi.mock('@/lib/api/client', () => ({
  apiFetch: (...args: unknown[]) => apiFetch(...args),
}));

vi.mock('@/lib/auth/tokens', () => ({
  getAccessToken: () => null,
}));

vi.mock('@/lib/api/portal', () => ({
  fetchPortalProducts: (...args: unknown[]) => fetchPortalProducts(...args),
}));

describe('fetchPortalPopularProducts — Phase 71I contract', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('given_api_failure_when_fetch_then_falls_back_to_catalog_products', async () => {
    apiFetch.mockRejectedValueOnce(new Error('not found'));
    fetchPortalProducts.mockResolvedValueOnce({
      data: [{ id: 'p1', name: 'Snack', sku: 'S1', priceAmount: 100, priceCurrency: 'BRL' }],
    });

    const products = await fetchPortalPopularProducts(8);
    expect(products).toHaveLength(1);
    expect(products[0]?.name).toBe('Snack');
  });

  it('given_popular_api_data_when_fetch_then_returns_api_list', async () => {
    apiFetch.mockResolvedValueOnce({
      data: [{ id: 'pop-1', name: 'Popular', sku: 'P1', priceAmount: 200, priceCurrency: 'BRL' }],
    });

    const products = await fetchPortalPopularProducts(12);
    expect(products[0]?.id).toBe('pop-1');
    expect(fetchPortalProducts).not.toHaveBeenCalled();
  });
});

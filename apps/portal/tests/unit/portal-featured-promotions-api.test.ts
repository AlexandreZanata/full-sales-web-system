import { beforeEach, describe, expect, it, vi } from 'vitest';

import { fetchPortalFeaturedProducts } from '@/lib/api/portalFeatured';
import { fetchPortalPromotions } from '@/lib/api/portalPromotions';

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

describe('fetchPortalFeaturedProducts — Phase 71G contract', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('given_api_failure_when_fetch_then_falls_back_to_catalog_products', async () => {
    apiFetch.mockRejectedValueOnce(new Error('not found'));
    fetchPortalProducts.mockResolvedValueOnce({
      data: [{ id: 'p1', name: 'Snack', sku: 'S1', priceAmount: 100, priceCurrency: 'BRL' }],
    });

    const products = await fetchPortalFeaturedProducts(8);
    expect(products).toHaveLength(1);
    expect(products[0]?.name).toBe('Snack');
  });

  it('given_featured_api_data_when_fetch_then_returns_api_list', async () => {
    apiFetch.mockResolvedValueOnce({
      data: [{ id: 'f1', name: 'Featured', sku: 'F1', priceAmount: 200, priceCurrency: 'BRL' }],
    });

    const products = await fetchPortalFeaturedProducts(12);
    expect(products[0]?.id).toBe('f1');
    expect(fetchPortalProducts).not.toHaveBeenCalled();
  });
});

describe('fetchPortalPromotions — Phase 71H contract', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('given_api_failure_when_fetch_then_returns_demo_promotions', async () => {
    apiFetch.mockRejectedValueOnce(new Error('not found'));

    const promotions = await fetchPortalPromotions(2);
    expect(promotions).toHaveLength(2);
    expect(promotions[0]).toEqual(
      expect.objectContaining({ headline: 'Tasty Burger', background: 'yellow' }),
    );
  });

  it('given_api_data_when_fetch_then_returns_api_list', async () => {
    apiFetch.mockResolvedValueOnce({
      data: [
        {
          id: 'live-1',
          headline: 'Live promo',
          discountText: '10% OFF',
          background: 'green',
        },
      ],
    });

    const promotions = await fetchPortalPromotions(4);
    expect(promotions[0]?.headline).toBe('Live promo');
  });
});

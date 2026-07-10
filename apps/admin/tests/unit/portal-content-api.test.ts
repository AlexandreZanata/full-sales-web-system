import { describe, expect, it, vi } from 'vitest';

import {
  createPortalBanner,
  fetchPortalBanners,
  fetchPortalPromotions,
} from '@/lib/api/portalContent';

const apiFetch = vi.fn();
const apiPost = vi.fn();

vi.mock('@/lib/api/client', () => ({
  apiFetch: (...args: unknown[]) => apiFetch(...args),
  apiPost: (...args: unknown[]) => apiPost(...args),
  apiPatch: vi.fn(),
  apiDelete: vi.fn(),
}));

describe('portalContent API — Phase 71L contract', () => {
  it('given_banners_when_fetch_then_returns_data_array', async () => {
    apiFetch.mockResolvedValueOnce({
      data: [{ id: 'b1', placement: 'hero', imageFileId: 'f1', sortOrder: 0, active: true }],
    });

    const banners = await fetchPortalBanners();
    expect(banners[0]?.placement).toBe('hero');
  });

  it('given_create_banner_when_post_then_returns_banner', async () => {
    apiPost.mockResolvedValueOnce({
      id: 'b2',
      placement: 'hero',
      imageFileId: 'file-1',
      sortOrder: 1,
      active: true,
    });

    const banner = await createPortalBanner({ imageFileId: 'file-1' });
    expect(banner.id).toBe('b2');
  });

  it('given_promotions_when_fetch_then_returns_data_array', async () => {
    apiFetch.mockResolvedValueOnce({
      data: [
        {
          id: 'p1',
          headline: 'Deal',
          discountText: '10% OFF',
          background: 'yellow',
          sortOrder: 0,
          active: true,
        },
      ],
    });

    const promotions = await fetchPortalPromotions();
    expect(promotions[0]?.headline).toBe('Deal');
  });
});

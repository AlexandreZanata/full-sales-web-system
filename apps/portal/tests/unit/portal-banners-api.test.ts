import { describe, expect, it, vi } from 'vitest';

import { fetchPortalBanners } from '@/lib/api/portal';

const apiFetch = vi.fn();
const fetchSettings = vi.fn();

vi.mock('@/lib/api/client', () => ({
  ApiError: class ApiError extends Error {
    status: number;
    constructor(status: number) {
      super('api error');
      this.status = status;
    }
  },
  apiFetch: (...args: unknown[]) => apiFetch(...args),
}));

vi.mock('@/lib/auth/tokens', () => ({
  getAccessToken: () => null,
}));

vi.mock('@/lib/api/settings', () => ({
  fetchSettings: () => fetchSettings(),
}));

describe('fetchPortalBanners — Phase 71D contract', () => {
  it('given_api_failure_when_fetch_then_returns_demo_banner', async () => {
    apiFetch.mockRejectedValueOnce(new Error('not found'));
    fetchSettings.mockRejectedValueOnce(new Error('offline'));

    const banners = await fetchPortalBanners('hero');
    expect(banners).toEqual([
      expect.objectContaining({ id: 'demo-hero-1', imageUrl: '/demo/hero-banner.svg' }),
    ]);
  });

  it('given_settings_hero_banners_when_api_empty_then_uses_settings', async () => {
    apiFetch.mockResolvedValueOnce({ data: [] });
    fetchSettings.mockResolvedValueOnce({
      heroBanners: [{ id: 'settings-1', imageUrl: '/custom.svg' }],
    });

    const banners = await fetchPortalBanners('hero');
    expect(banners[0]?.imageUrl).toBe('/custom.svg');
  });

  it('given_fetch_when_success_then_uses_public_banners_route', async () => {
    apiFetch.mockResolvedValueOnce({
      data: [{ id: 'b1', imageUrl: '/v1/public/media/b1/content', altText: 'FoodKing hero' }],
    });

    const banners = await fetchPortalBanners('hero');

    expect(apiFetch).toHaveBeenCalledWith('/public/banners?placement=hero&limit=10', {
      skipAuth: true,
    });
    expect(banners[0]?.imageUrl).toBe('/v1/public/media/b1/content');
  });
});

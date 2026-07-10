import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { screen } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';

import { HeroBannerCarousel } from '@/components/catalog/home/HeroBannerCarousel';
import { useHeroBanners } from '@/lib/catalog/useHeroBanners';
import { renderWithI18n } from '../helpers/renderWithI18n';

vi.mock('swiper/react', () => ({
  Swiper: ({ children }: { children: React.ReactNode }) => (
    <div data-testid="swiper-mock">{children}</div>
  ),
  SwiperSlide: ({ children }: { children: React.ReactNode }) => <div>{children}</div>,
}));

vi.mock('swiper/modules', () => ({
  Autoplay: {},
  Pagination: {},
}));

vi.mock('@/lib/catalog/useHeroBanners', () => ({
  useHeroBanners: vi.fn(),
}));

function renderHero() {
  const queryClient = new QueryClient();
  return renderWithI18n(
    <QueryClientProvider client={queryClient}>
      <HeroBannerCarousel />
    </QueryClientProvider>,
  );
}

describe('HeroBannerCarousel — Phase 71D contract', () => {
  it('given_loading_when_rendered_then_shows_skeleton_testid', () => {
    vi.mocked(useHeroBanners).mockReturnValue({
      isLoading: true,
      data: undefined,
    } as ReturnType<typeof useHeroBanners>);

    renderHero();
    expect(screen.getByTestId('hero-banner')).toHaveAttribute('aria-busy', 'true');
  });

  it('given_banner_when_loaded_then_renders_image', () => {
    vi.mocked(useHeroBanners).mockReturnValue({
      isLoading: false,
      data: [{ id: 'b1', imageUrl: '/demo/hero-banner.svg', altText: 'Welcome' }],
    } as ReturnType<typeof useHeroBanners>);

    renderHero();
    expect(screen.getByRole('img', { name: 'Welcome' })).toHaveAttribute(
      'src',
      '/demo/hero-banner.svg',
    );
    expect(screen.getByRole('img', { name: 'Welcome' })).toHaveAttribute('fetchpriority', 'high');
    expect(screen.getByRole('img', { name: 'Welcome' })).toHaveAttribute(
      'sizes',
      '(min-width: 1152px) 1152px, 100vw',
    );
  });

  it('given_zero_banners_when_loaded_then_hides_section', () => {
    vi.mocked(useHeroBanners).mockReturnValue({
      isLoading: false,
      data: [],
    } as ReturnType<typeof useHeroBanners>);

    renderHero();
    expect(screen.queryByTestId('hero-banner')).not.toBeInTheDocument();
  });
});

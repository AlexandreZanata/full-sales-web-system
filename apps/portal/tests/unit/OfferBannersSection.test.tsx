import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { screen } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';

import { OfferBannersSection } from '@/components/catalog/home/OfferBannersSection';
import { usePromotions } from '@/lib/catalog/usePromotions';
import { renderWithI18n } from '../helpers/renderWithI18n';

vi.mock('@tanstack/react-router', () => ({
  Link: ({
    children,
    to,
    search,
    className,
  }: {
    children: React.ReactNode;
    to: string;
    search?: { category?: string };
    className?: string;
  }) => (
    <a href={`${to}?category=${search?.category ?? ''}`} className={className}>
      {children}
    </a>
  ),
}));

vi.mock('@/lib/catalog/usePromotions', () => ({
  usePromotions: vi.fn(),
}));

function renderOffers() {
  const queryClient = new QueryClient();
  return renderWithI18n(
    <QueryClientProvider client={queryClient}>
      <OfferBannersSection />
    </QueryClientProvider>,
  );
}

describe('OfferBannersSection — Phase 71H contract', () => {
  it('given_loading_when_rendered_then_shows_skeleton_with_testid', () => {
    vi.mocked(usePromotions).mockReturnValue({
      isLoading: true,
      data: undefined,
    } as ReturnType<typeof usePromotions>);

    renderOffers();
    expect(screen.getByTestId('offer-banners')).toHaveAttribute('aria-busy', 'true');
  });

  it('given_promotions_when_loaded_then_renders_pastel_cards_and_order_cta', () => {
    vi.mocked(usePromotions).mockReturnValue({
      isLoading: false,
      data: [
        {
          id: 'p1',
          headline: 'Tasty Burger',
          discountText: '30% OFF',
          categorySlug: 'snacks',
          background: 'yellow',
        },
        {
          id: 'p2',
          headline: 'Fresh Salad',
          discountText: '15% OFF',
          categorySlug: 'bebidas',
          background: 'green',
        },
      ],
    } as ReturnType<typeof usePromotions>);

    renderOffers();

    const section = screen.getByTestId('offer-banners');
    expect(section).toHaveAttribute('id', 'offers');
    expect(screen.getByText('Tasty Burger')).toBeInTheDocument();
    expect(screen.getByText('30% OFF')).toBeInTheDocument();
    expect(screen.getAllByText('Pedir agora')).toHaveLength(2);
    expect(screen.getByRole('link', { name: /tasty burger/i })).toHaveAttribute(
      'href',
      '/?category=snacks',
    );
  });

  it('given_zero_promotions_when_loaded_then_hides_section', () => {
    vi.mocked(usePromotions).mockReturnValue({
      isLoading: false,
      data: [],
    } as ReturnType<typeof usePromotions>);

    renderOffers();
    expect(screen.queryByTestId('offer-banners')).not.toBeInTheDocument();
  });
});

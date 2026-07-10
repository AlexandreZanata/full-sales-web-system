import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { screen } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';

import { FeaturedItemsSection } from '@/components/catalog/home/FeaturedItemsSection';
import { useFeaturedProducts } from '@/lib/catalog/useFeaturedProducts';
import type { PortalProduct } from '@/lib/api/types';
import { renderWithI18n } from '../helpers/renderWithI18n';

vi.mock('@tanstack/react-router', () => ({
  useNavigate: () => vi.fn(),
}));

vi.mock('@/cart/CartProvider', () => ({
  useCart: () => ({ addProduct: vi.fn() }),
}));

vi.mock('@/lib/catalog/useFeaturedProducts', () => ({
  useFeaturedProducts: vi.fn(),
}));

const sampleProduct: PortalProduct = {
  id: 'prod-1',
  name: 'Cola 2L',
  sku: 'BEV-001',
  priceAmount: 899,
  priceCurrency: 'BRL',
  description: 'Cold drink',
};

function renderFeatured() {
  const queryClient = new QueryClient();
  return renderWithI18n(
    <QueryClientProvider client={queryClient}>
      <FeaturedItemsSection />
    </QueryClientProvider>,
  );
}

describe('FeaturedItemsSection — Phase 71G contract', () => {
  it('given_loading_when_rendered_then_shows_skeleton_with_testid', () => {
    vi.mocked(useFeaturedProducts).mockReturnValue({
      isLoading: true,
      data: undefined,
    } as ReturnType<typeof useFeaturedProducts>);

    renderFeatured();
    expect(screen.getByTestId('featured-items')).toHaveAttribute('aria-busy', 'true');
  });

  it('given_products_when_loaded_then_renders_title_and_grid_cards', () => {
    vi.mocked(useFeaturedProducts).mockReturnValue({
      isLoading: false,
      data: [sampleProduct],
    } as ReturnType<typeof useFeaturedProducts>);

    renderFeatured();
    expect(screen.getByTestId('featured-items')).toBeInTheDocument();
    expect(screen.getByRole('heading', { name: 'Itens em destaque' })).toBeInTheDocument();
    expect(screen.getByText('Cola 2L')).toBeInTheDocument();
    expect(screen.getByText('Cold drink')).toBeInTheDocument();
  });

  it('given_zero_products_when_loaded_then_hides_section', () => {
    vi.mocked(useFeaturedProducts).mockReturnValue({
      isLoading: false,
      data: [],
    } as ReturnType<typeof useFeaturedProducts>);

    renderFeatured();
    expect(screen.queryByTestId('featured-items')).not.toBeInTheDocument();
  });
});

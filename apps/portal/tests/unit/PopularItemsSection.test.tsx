import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { screen } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';

import { PopularItemsSection } from '@/components/catalog/home/PopularItemsSection';
import { usePopularProducts } from '@/lib/catalog/usePopularProducts';
import type { PortalProduct } from '@/lib/api/types';
import { renderWithI18n } from '../helpers/renderWithI18n';

vi.mock('@tanstack/react-router', () => ({
  useNavigate: () => vi.fn(),
}));

vi.mock('@/cart/CartProvider', () => ({
  useCart: () => ({ addProduct: vi.fn() }),
}));

vi.mock('@/lib/catalog/usePopularProducts', () => ({
  usePopularProducts: vi.fn(),
}));

const sampleProduct: PortalProduct = {
  id: 'prod-2',
  name: 'Chips Original',
  sku: 'SNK-001',
  priceAmount: 599,
  priceCurrency: 'BRL',
  description: 'Crispy chips',
};

function renderPopular() {
  const queryClient = new QueryClient();
  return renderWithI18n(
    <QueryClientProvider client={queryClient}>
      <PopularItemsSection />
    </QueryClientProvider>,
  );
}

describe('PopularItemsSection — Phase 71I contract', () => {
  it('given_loading_when_rendered_then_shows_skeleton_with_testid', () => {
    vi.mocked(usePopularProducts).mockReturnValue({
      isLoading: true,
      data: undefined,
    } as ReturnType<typeof usePopularProducts>);

    renderPopular();
    expect(screen.getByTestId('popular-items')).toHaveAttribute('aria-busy', 'true');
  });

  it('given_products_when_loaded_then_renders_title_and_list_cards', () => {
    vi.mocked(usePopularProducts).mockReturnValue({
      isLoading: false,
      data: [sampleProduct],
    } as ReturnType<typeof usePopularProducts>);

    renderPopular();
    expect(screen.getByTestId('popular-items')).toBeInTheDocument();
    expect(screen.getByRole('heading', { name: 'Itens mais populares' })).toBeInTheDocument();
    expect(screen.getByText('Chips Original')).toBeInTheDocument();
    expect(screen.getByText('Crispy chips')).toBeInTheDocument();
  });

  it('given_zero_products_when_loaded_then_hides_section', () => {
    vi.mocked(usePopularProducts).mockReturnValue({
      isLoading: false,
      data: [],
    } as ReturnType<typeof usePopularProducts>);

    renderPopular();
    expect(screen.queryByTestId('popular-items')).not.toBeInTheDocument();
  });
});

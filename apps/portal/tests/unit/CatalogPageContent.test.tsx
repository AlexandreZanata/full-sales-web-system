import { screen } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';

import { CatalogPageContent } from '@/components/catalog/CatalogPageContent';
import { renderWithI18n } from '../helpers/renderWithI18n';

vi.mock('@tanstack/react-router', () => ({
  Link: ({ children, to, search }: { children: React.ReactNode; to: string; search?: object }) => (
    <a href={`${to}${search ? '?home=1' : ''}`}>{children}</a>
  ),
  useNavigate: () => vi.fn(),
}));

vi.mock('@/cart/CartProvider', () => ({
  useCart: () => ({ addProduct: vi.fn() }),
}));

vi.mock('@/lib/catalog/useCatalogCategories', () => ({
  useCatalogCategories: () => ({
    isLoading: false,
    data: [{ id: '1', name: 'Bebidas', slug: 'bebidas', sortOrder: 0, active: true }],
  }),
}));

vi.mock('@tanstack/react-query', async (importOriginal) => {
  const actual = await importOriginal<typeof import('@tanstack/react-query')>();
  return {
    ...actual,
    useInfiniteQuery: () => ({
      isLoading: false,
      isError: false,
      data: {
        pages: [
          {
            name: 'Bebidas',
            products: [
              {
                id: 'p1',
                name: 'Cola 2L',
                sku: 'SEED-001',
                priceAmount: 850,
                priceCurrency: 'BRL',
              },
            ],
            pagination: { has_more: false, next_cursor: null, limit: 50 },
          },
        ],
      },
      hasNextPage: false,
      isFetchingNextPage: false,
      fetchNextPage: vi.fn(),
      refetch: vi.fn(),
    }),
  };
});

describe('CatalogPageContent — Phase 71K contract', () => {
  it('renders_menu_testid_back_link_and_category_bar', () => {
    renderWithI18n(<CatalogPageContent categoryParam="bebidas" initialSearch="" />);

    expect(screen.getByTestId('catalog-menu')).toBeInTheDocument();
    expect(screen.getByRole('link', { name: 'Voltar ao início' })).toBeInTheDocument();
    expect(screen.getByRole('tab', { name: /bebidas/i })).toHaveClass(
      'catalog-category-chip--active',
    );
    expect(screen.getByText('Cola 2L')).toBeInTheDocument();
  });
});

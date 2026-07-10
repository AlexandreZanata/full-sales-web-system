import { screen } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';

import { HomeCategorySection } from '@/components/catalog/home/HomeCategorySection';
import { renderWithI18n } from '../helpers/renderWithI18n';

vi.mock('@tanstack/react-router', () => ({
  Link: ({
    children,
    to,
    className,
  }: {
    children: React.ReactNode;
    to: string;
    className?: string;
  }) => (
    <a href={to} className={className}>
      {children}
    </a>
  ),
  useNavigate: () => vi.fn(),
}));

vi.mock('@/lib/catalog/useCatalogCategories', () => ({
  useCatalogCategories: () => ({
    isLoading: false,
    data: [
      { id: '1', name: 'Bebidas', slug: 'bebidas', sortOrder: 0, active: true },
      { id: '2', name: 'Snacks', slug: 'snacks', sortOrder: 1, active: true },
    ],
  }),
}));

describe('HomeCategorySection — Phase 71E contract', () => {
  it('renders_title_view_all_and_category_row', () => {
    renderWithI18n(<HomeCategorySection />);

    expect(screen.getByTestId('home-categories')).toBeInTheDocument();
    expect(screen.getByRole('heading', { name: 'Nosso cardápio' })).toBeInTheDocument();
    expect(screen.getByRole('link', { name: 'Ver tudo' })).toHaveClass('catalog-view-all-pill');
    expect(screen.getByRole('tab', { name: /bebidas/i })).toHaveClass(
      'catalog-category-chip--home',
    );
  });
});

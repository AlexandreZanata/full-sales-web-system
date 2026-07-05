import { screen } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';

import { CategoryBar } from '@/components/catalog/CategoryBar';
import type { PortalCategory } from '@/lib/api/types';
import { renderWithI18n } from '../helpers/renderWithI18n';

const categories: PortalCategory[] = [
  { id: '1', name: 'Bebidas', slug: 'bebidas', sortOrder: 0, active: true },
  { id: '2', name: 'Snacks', slug: 'snacks', sortOrder: 1, active: true },
];

describe('CategoryBar — Phase 45 contract', () => {
  it('highlights the active category slug', () => {
    renderWithI18n(<CategoryBar categories={categories} activeSlug="bebidas" onSelect={vi.fn()} />);

    const activeTab = screen.getByRole('tab', { name: /bebidas/i });
    expect(activeTab).toHaveAttribute('aria-current', 'true');
    expect(activeTab).toHaveClass('catalog-category-chip--active');
  });

  it('calls onSelect when a category chip is clicked', () => {
    const onSelect = vi.fn();
    renderWithI18n(
      <CategoryBar categories={categories} activeSlug="bebidas" onSelect={onSelect} />,
    );

    screen.getByRole('tab', { name: /snacks/i }).click();
    expect(onSelect).toHaveBeenCalledWith('snacks');
  });
});

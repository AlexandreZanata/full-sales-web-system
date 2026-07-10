import { screen } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';

import { CatalogHomePage } from '@/components/catalog/home/CatalogHomePage';
import { renderWithI18n } from '../helpers/renderWithI18n';

vi.mock('@/components/catalog/home/HeroBannerCarousel', () => ({
  HeroBannerCarousel: () => <section data-testid="hero-banner" />,
}));

vi.mock('@/components/catalog/home/HomeCategorySection', () => ({
  HomeCategorySection: () => <section data-testid="home-categories" />,
}));

vi.mock('@/components/catalog/home/FeaturedItemsSection', () => ({
  FeaturedItemsSection: () => <section data-testid="featured-items" />,
}));

vi.mock('@/components/catalog/home/OfferBannersSection', () => ({
  OfferBannersSection: () => <section data-testid="offer-banners" />,
}));

vi.mock('@/components/catalog/home/PopularItemsSection', () => ({
  PopularItemsSection: () => <section data-testid="popular-items" />,
}));

describe('CatalogHomePage — Phase 71J contract', () => {
  it('renders_home_sections_in_design_order', () => {
    renderWithI18n(<CatalogHomePage />);

    const home = screen.getByTestId('catalog-home-page');
    const sections = Array.from(home.children)
      .filter((child) => child.tagName === 'SECTION')
      .map((child) => child.getAttribute('data-testid'));

    expect(sections).toEqual([
      'hero-banner',
      'home-categories',
      'featured-items',
      'offer-banners',
      'popular-items',
    ]);
  });
});

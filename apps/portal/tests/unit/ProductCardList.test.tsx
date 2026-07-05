import { screen } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';

import { ProductCardList } from '@/components/catalog/ProductCardList';
import type { PortalProduct } from '@/lib/api/types';
import { renderWithI18n } from '../helpers/renderWithI18n';

const product: PortalProduct = {
  id: 'prod-1',
  name: 'Premium Widget',
  sku: 'SEED-001',
  categoryName: 'Bebidas',
  priceAmount: 250_000,
  priceCurrency: 'BRL',
};

describe('ProductCardList — mobile list card contract', () => {
  it('renders prominent price, combined metadata, and triggers onAddToCart', () => {
    const onAddToCart = vi.fn();
    renderWithI18n(
      <ProductCardList
        product={product}
        onAddToCart={onAddToCart}
        addToCartLabel="Add to cart"
        skuLabel="SKU"
      />,
    );

    expect(screen.getByRole('heading', { name: 'Premium Widget' })).toBeInTheDocument();
    expect(screen.getByText('SKU: SEED-001 · Bebidas')).toBeInTheDocument();
    expect(screen.getByText('R$ 2.500,00')).toHaveClass('catalog-price--prominent');
    screen.getByRole('button', { name: 'Add to cart' }).click();
    expect(onAddToCart).toHaveBeenCalledWith(product);
  });

  it('opens product detail when title is clicked', () => {
    const onOpenDetail = vi.fn();
    renderWithI18n(
      <ProductCardList
        product={product}
        onAddToCart={vi.fn()}
        onOpenDetail={onOpenDetail}
        addToCartLabel="Add to cart"
        skuLabel="SKU"
      />,
    );

    screen.getByRole('heading', { name: 'Premium Widget' }).click();
    expect(onOpenDetail).toHaveBeenCalledWith(product);
  });

  it('renders SKU-only metadata when category is missing', () => {
    const { categoryName: _category, ...productWithoutCategory } = product;
    renderWithI18n(
      <ProductCardList
        product={productWithoutCategory}
        onAddToCart={vi.fn()}
        addToCartLabel="Add to cart"
        skuLabel="SKU"
      />,
    );

    expect(screen.getByText('SKU: SEED-001')).toBeInTheDocument();
    expect(screen.queryByText(/Bebidas/)).not.toBeInTheDocument();
  });
});

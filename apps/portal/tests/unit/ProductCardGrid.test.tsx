import { screen } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';

import { ProductCardGrid } from '@/components/catalog/ProductCardGrid';
import type { PortalProduct } from '@/lib/api/types';
import { renderWithI18n } from '../helpers/renderWithI18n';

const product: PortalProduct = {
  id: 'prod-1',
  name: 'Premium Widget',
  sku: 'SKU-001',
  priceAmount: 1050,
  priceCurrency: 'BRL',
};

describe('ProductCardGrid — Phase 45 contract', () => {
  it('renders formatted price and triggers onAddToCart', () => {
    const onAddToCart = vi.fn();
    renderWithI18n(
      <ProductCardGrid
        product={product}
        onAddToCart={onAddToCart}
        addToCartLabel="Add to cart"
        skuLabel="SKU"
      />,
    );

    expect(screen.getByText('R$ 10,50')).toBeInTheDocument();
    screen.getByRole('button', { name: 'Add to cart' }).click();
    expect(onAddToCart).toHaveBeenCalledWith(product);
  });
});

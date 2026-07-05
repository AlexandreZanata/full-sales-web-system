import { screen } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';

import { ProductDetailActions } from '@/components/catalog/ProductDetailActions';
import type { PortalProductDetail } from '@/lib/api/types';
import { renderWithI18n } from '../helpers/renderWithI18n';

const product: PortalProductDetail = {
  id: 'prod-1',
  name: 'Premium Widget',
  sku: 'SKU-001',
  priceAmount: 1050,
  priceCurrency: 'BRL',
  unitOfMeasure: 'UN',
  imageUrls: [],
};

vi.mock('@tanstack/react-router', () => ({
  useNavigate: () => vi.fn(),
}));

vi.mock('@/cart/CartProvider', () => ({
  useCart: () => ({ addProduct: vi.fn() }),
}));

describe('ProductDetailActions — Phase 50 contract', () => {
  it('renders contact seller link when phone configured', () => {
    renderWithI18n(
      <ProductDetailActions
        product={product}
        salesContactPhone="5511987654321"
        productUrl="https://portal.example/products/prod-1"
      />,
    );

    const link = screen.getByRole('link', { name: /Contact seller|Falar com vendedor/ });
    expect(link).toHaveAttribute('href', expect.stringContaining('https://wa.me/5511987654321'));
  });

  it('disables contact seller when phone missing', () => {
    renderWithI18n(
      <ProductDetailActions
        product={product}
        productUrl="https://portal.example/products/prod-1"
      />,
    );

    expect(
      screen.getByRole('button', { name: /Contact seller|Falar com vendedor/ }),
    ).toBeDisabled();
  });
});

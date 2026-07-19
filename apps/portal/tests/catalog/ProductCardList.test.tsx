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
  description: 'Cold beverage',
};

describe('ProductCardList — Phase 71F contract', () => {
  it('renders_description_pill_add_and_prominent_price', () => {
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
    expect(screen.getByText('Cold beverage')).toBeInTheDocument();
    expect(screen.getByText('R$ 2.500,00')).toHaveClass('catalog-price--prominent');
    const addButton = screen.getByRole('button', { name: 'Add to cart' });
    expect(addButton).toHaveClass('catalog-add-pill-btn');
    addButton.click();
    expect(onAddToCart).toHaveBeenCalledWith(product);
  });

  it('opens_product_detail_when_card_body_clicked', () => {
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

    screen.getByText('Cold beverage').click();
    expect(onOpenDetail).toHaveBeenCalledWith(product);
  });
});

import { screen, within } from '@testing-library/react';
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
  description: '<p>Crispy snack</p>',
  compareAtPrice: 1500,
};

describe('ProductCardGrid — Phase 71F contract', () => {
  it('renders_description_pill_add_and_compare_price', () => {
    const onAddToCart = vi.fn();
    renderWithI18n(
      <ProductCardGrid
        product={product}
        onAddToCart={onAddToCart}
        addToCartLabel="Add to cart"
        skuLabel="SKU"
      />,
    );

    expect(screen.getByText('Crispy snack')).toBeInTheDocument();
    expect(screen.getByText('R$ 10,50')).toBeInTheDocument();
    expect(screen.getByText('R$ 15,00')).toHaveClass('line-through');
    const addButton = screen.getByRole('button', { name: 'Add to cart' });
    expect(addButton).toHaveClass('catalog-add-pill-btn');
    expect(screen.getByText('Add')).toBeInTheDocument();
    addButton.click();
    expect(onAddToCart).toHaveBeenCalledWith(product);
  });

  it('opens_info_dialog_when_info_icon_clicked', async () => {
    renderWithI18n(
      <ProductCardGrid
        product={product}
        onAddToCart={vi.fn()}
        addToCartLabel="Add to cart"
        skuLabel="SKU"
      />,
    );

    screen.getByRole('button', { name: 'Informações do produto' }).click();
    const dialog = await screen.findByRole('dialog');
    expect(within(dialog).getByText('Crispy snack')).toBeInTheDocument();
  });
});

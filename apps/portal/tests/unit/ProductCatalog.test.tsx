import { screen, waitFor } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';

import { ProductCatalog } from '@/components/catalog/ProductCatalog';
import type { PortalProduct } from '@/lib/api/types';
import { renderWithI18n } from '../helpers/renderWithI18n';

const products: PortalProduct[] = [
  {
    id: 'prod-1',
    name: 'Premium Widget',
    sku: 'SKU-001',
    priceAmount: 1050,
    priceCurrency: 'BRL',
  },
];

describe('ProductCatalog — Phase 45 contract', () => {
  it('switches between grid and list layouts', async () => {
    renderWithI18n(
      <ProductCatalog
        products={products}
        onAddToCart={vi.fn()}
        emptyTitle="No products"
        addToCartLabel="Add to cart"
        skuLabel="SKU"
        listViewLabel="List view"
        gridViewLabel="Grid view"
      />,
    );

    expect(screen.getByTestId('catalog-product-grid')).toBeInTheDocument();

    screen.getByRole('button', { name: 'List view' }).click();
    await waitFor(() => {
      expect(screen.getByTestId('catalog-product-list')).toBeInTheDocument();
    });
    expect(screen.queryByTestId('catalog-product-grid')).not.toBeInTheDocument();
  });
});

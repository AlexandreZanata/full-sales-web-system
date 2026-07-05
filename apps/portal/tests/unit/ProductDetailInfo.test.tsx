import { screen } from '@testing-library/react';
import { describe, expect, it } from 'vitest';

import { ProductDetailInfo } from '@/components/catalog/ProductDetailInfo';
import type { PortalProductDetail } from '@/lib/api/types';
import { renderWithI18n } from '../helpers/renderWithI18n';

const product: PortalProductDetail = {
  id: 'prod-1',
  name: 'Premium Widget',
  sku: 'SKU-001',
  priceAmount: 1050,
  priceCurrency: 'BRL',
  unitOfMeasure: 'CX',
  imageUrls: [],
  categoryName: 'Snacks',
  description: 'Crispy and delicious.',
};

describe('ProductDetailInfo — Phase 49 contract', () => {
  it('shows unit of measure in specs table', () => {
    renderWithI18n(<ProductDetailInfo product={product} />);

    expect(screen.getByText(/Unit of measure|Unidade de medida/)).toBeInTheDocument();
    expect(screen.getByText('CX')).toBeInTheDocument();
    expect(screen.getByText('Crispy and delicious.')).toBeInTheDocument();
  });
});

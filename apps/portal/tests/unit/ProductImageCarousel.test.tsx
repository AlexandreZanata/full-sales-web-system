import { screen } from '@testing-library/react';
import { describe, expect, it } from 'vitest';

import { ProductImageCarousel } from '@/components/catalog/ProductImageCarousel';
import { renderWithI18n } from '../helpers/renderWithI18n';

const slides = [
  { url: 'https://cdn.example/a.jpg', alt: 'Widget A' },
  { url: 'https://cdn.example/b.jpg', alt: 'Widget B' },
  { url: 'https://cdn.example/c.jpg', alt: 'Widget C' },
];

describe('ProductImageCarousel — Phase 49 contract', () => {
  it('renders N slides with matching dot indicators', () => {
    renderWithI18n(<ProductImageCarousel slides={slides} />);

    expect(screen.getByRole('img', { name: 'Widget A' })).toBeInTheDocument();
    expect(screen.getAllByRole('button', { name: /Go to image|Ir para imagem/ })).toHaveLength(3);
    expect(screen.getByRole('button', { name: /Previous image|Imagem anterior/ })).toBeInTheDocument();
    expect(screen.getByRole('button', { name: /Next image|Próxima imagem/ })).toBeInTheDocument();
  });

  it('hides carousel chrome for a single slide', () => {
    renderWithI18n(<ProductImageCarousel slides={[slides[0] as (typeof slides)[number]]} />);

    expect(screen.getByRole('img', { name: 'Widget A' })).toBeInTheDocument();
    expect(screen.queryByRole('button', { name: /Previous image|Imagem anterior/ })).not.toBeInTheDocument();
    expect(screen.queryByRole('button', { name: /Go to image|Ir para imagem/ })).not.toBeInTheDocument();
  });
});

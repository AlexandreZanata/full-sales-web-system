import { screen } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';

import { PortalFooter } from '@/components/layout/PortalFooter';
import { renderWithI18n } from '../helpers/renderWithI18n';

vi.mock('@tanstack/react-router', () => ({
  Link: ({ children, to, ...props }: { children: React.ReactNode; to: string }) => (
    <a href={to} {...props}>
      {children}
    </a>
  ),
}));

vi.mock('@/lib/settings/useSiteSettings', () => ({
  useSiteSettings: () => ({
    data: {
      displayName: 'Zé Fominha',
      salesContactPhone: '5511987654321',
    },
  }),
}));

vi.mock('@/lib/catalog/useCatalogCategories', () => ({
  useCatalogCategories: () => ({
    data: [{ id: '1', name: 'Bebidas', slug: 'bebidas', sortOrder: 1, active: true }],
  }),
}));

describe('PortalFooter — Phase 71C contract', () => {
  it('renders_newsletter_contact_and_copyright_on_primary_band', () => {
    renderWithI18n(<PortalFooter />);

    expect(screen.getByRole('contentinfo')).toHaveClass('portal-footer');
    expect(screen.getByText('Links úteis')).toBeInTheDocument();
    expect(screen.getByText('Contato')).toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'Assinar' })).toBeInTheDocument();
    expect(screen.getByText(/Todos os direitos reservados/)).toBeInTheDocument();
    expect(screen.getByRole('link', { name: '5511987654321' })).toBeInTheDocument();
  });
});

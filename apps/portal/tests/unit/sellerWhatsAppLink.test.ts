import { describe, expect, it } from 'vitest';

import { buildSellerWhatsAppLink } from '@/lib/contact/sellerWhatsAppLink';

describe('buildSellerWhatsAppLink — Phase 50 contract', () => {
  it('given_phone_and_product_when_build_then_valid_wa_me_url', () => {
    const url = buildSellerWhatsAppLink(
      '+55 11 98765-4321',
      { name: 'Seed Widget', sku: 'SKU-001' },
      'https://portal.example/products/prod-1?category=snacks',
    );

    expect(url).toMatch(/^https:\/\/wa\.me\/5511987654321\?text=/);
    expect(decodeURIComponent(url)).toContain('Seed Widget');
    expect(decodeURIComponent(url)).toContain('SKU-001');
    expect(decodeURIComponent(url)).toContain('https://portal.example/products/prod-1');
  });
});

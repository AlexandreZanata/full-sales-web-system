import { describe, expect, it } from 'vitest';

import {
  buildSellerWhatsAppLink,
  formatSellerWhatsAppMessage,
} from '@/lib/contact/sellerWhatsAppLink';

describe('buildSellerWhatsAppLink — Phase 50 contract', () => {
  it('given_phone_and_message_when_build_then_valid_wa_me_url', () => {
    const message = formatSellerWhatsAppMessage(
      'Olá, tenho interesse em {name} (SKU: {sku})\n{url}',
      { name: 'Seed Widget', sku: 'SKU-001' },
      'https://portal.example/products/prod-1?category=snacks',
    );
    const url = buildSellerWhatsAppLink('+55 11 98765-4321', message);

    expect(url).toMatch(/^https:\/\/wa\.me\/5511987654321\?text=/);
    const decoded = decodeURIComponent(url);
    expect(decoded).toContain('Olá, tenho interesse em Seed Widget');
    expect(decoded).toContain('SKU-001');
    expect(decoded).toContain('https://portal.example/products/prod-1');
  });
});

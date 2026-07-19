import { describe, expect, it } from 'vitest';

import { formatCartWhatsAppMessage } from '@/lib/contact/cartWhatsAppMessage';

describe('formatCartWhatsAppMessage', () => {
  it('lists_each_product_with_quantity', () => {
    const message = formatCartWhatsAppMessage(
      'Olá, tenho interesse nestes produtos:\n\n{items}\n',
      '- {name} — qtd: {qty}',
      [
        { name: 'Coca-Cola 2L', quantity: 2 },
        { name: 'Produto X', quantity: 1 },
      ],
    );
    expect(message).toContain('Olá, tenho interesse nestes produtos:');
    expect(message).toContain('- Coca-Cola 2L — qtd: 2');
    expect(message).toContain('- Produto X — qtd: 1');
  });
});

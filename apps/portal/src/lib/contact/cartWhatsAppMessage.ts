import type { CartLine } from '@/cart/CartProvider';

export type CartWhatsAppLine = Pick<CartLine, 'name' | 'quantity'>;

/** Builds cart interest body from locale templates (`{items}` / `{name}` / `{qty}`). */
export function formatCartWhatsAppMessage(
  introTemplate: string,
  itemTemplate: string,
  lines: CartWhatsAppLine[],
): string {
  const items = lines
    .map((line) =>
      itemTemplate.replaceAll('{name}', line.name).replaceAll('{qty}', String(line.quantity)),
    )
    .join('\n');
  return introTemplate.replaceAll('{items}', items).trimEnd();
}

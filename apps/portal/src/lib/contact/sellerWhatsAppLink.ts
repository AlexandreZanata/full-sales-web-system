import { isValidSalesContactPhone, normalizeSalesContactPhone } from '@/lib/contact/phone';

export type SellerWhatsAppProduct = {
  name: string;
  sku: string;
};

/** Fills `{name}`, `{sku}`, `{url}` in a locale-specific interest template. */
export function formatSellerWhatsAppMessage(
  template: string,
  product: SellerWhatsAppProduct,
  productUrl: string,
): string {
  return template
    .replaceAll('{name}', product.name)
    .replaceAll('{sku}', product.sku)
    .replaceAll('{url}', productUrl);
}

export function buildSellerWhatsAppLink(phone: string, message: string): string {
  const digits = normalizeSalesContactPhone(phone);
  if (!isValidSalesContactPhone(digits)) {
    throw new Error('Invalid sales contact phone');
  }

  const waPhone = digits.length <= 11 ? `55${digits}` : digits;
  return `https://wa.me/${waPhone}?text=${encodeURIComponent(message)}`;
}

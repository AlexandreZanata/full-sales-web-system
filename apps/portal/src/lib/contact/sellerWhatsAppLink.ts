import { isValidSalesContactPhone, normalizeSalesContactPhone } from '@/lib/contact/phone';

export type SellerWhatsAppProduct = {
  name: string;
  sku: string;
};

export function buildSellerWhatsAppLink(
  phone: string,
  product: SellerWhatsAppProduct,
  productUrl: string,
): string {
  const digits = normalizeSalesContactPhone(phone);
  if (!isValidSalesContactPhone(digits)) {
    throw new Error('Invalid sales contact phone');
  }

  const waPhone = digits.length <= 11 ? `55${digits}` : digits;
  const message = `Hi, I'm interested in ${product.name} (SKU: ${product.sku})\n${productUrl}`;
  return `https://wa.me/${waPhone}?text=${encodeURIComponent(message)}`;
}

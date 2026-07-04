import type { SaleStatus } from '@/lib/admin-tokens';

export const SALE_STATUSES: SaleStatus[] = ['Pending', 'Confirmed', 'Cancelled'];

export type SaleStatusFilter = SaleStatus | '';

export const SALE_STATUS_FILTERS: SaleStatusFilter[] = ['', ...SALE_STATUSES];

export const PAYMENT_METHODS = ['cash', 'pix', 'credit', 'debit'] as const;

export type PaymentMethodValue = (typeof PAYMENT_METHODS)[number];

export function isDeclaredPayment(method: string, received: boolean): boolean {
  if (received) return true;
  const normalized = method.trim().toLowerCase();
  return normalized.length > 0 && normalized !== 'notdeclared';
}

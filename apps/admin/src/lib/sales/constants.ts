import type { SaleStatus } from '@/lib/admin-tokens';

export const SALE_STATUSES: SaleStatus[] = ['Pending', 'Confirmed', 'Cancelled'];

export type SaleStatusFilter = SaleStatus | '';

export const SALE_STATUS_FILTER_LABELS: Record<SaleStatusFilter, string> = {
  '': 'All statuses',
  Pending: 'Pending',
  Confirmed: 'Confirmed',
  Cancelled: 'Cancelled',
};

export const PAYMENT_METHODS = ['cash', 'pix', 'credit', 'debit'] as const;

export type PaymentMethodValue = (typeof PAYMENT_METHODS)[number];

export const PAYMENT_METHOD_LABELS: Record<PaymentMethodValue, string> = {
  cash: 'Cash',
  pix: 'Pix',
  credit: 'Credit card',
  debit: 'Debit card',
};

/** Maps API PascalCase payment method to display label. */
export function paymentMethodLabel(value: string): string {
  const normalized = value.toLowerCase();
  if (normalized === 'cash') return 'Cash';
  if (normalized === 'pix') return 'Pix';
  if (normalized === 'credit') return 'Credit card';
  if (normalized === 'debit') return 'Debit card';
  if (normalized === 'notdeclared') return 'Not declared';
  return value;
}

export function isDeclaredPayment(method: string, received: boolean): boolean {
  if (received) return true;
  const normalized = method.trim().toLowerCase();
  return normalized.length > 0 && normalized !== 'notdeclared';
}

export function declaredPaymentLabel(method: string, received: boolean): string {
  if (!isDeclaredPayment(method, received)) {
    return 'Not declared';
  }
  const label = paymentMethodLabel(method);
  return received ? `${label} · received` : `${label} · pending`;
}

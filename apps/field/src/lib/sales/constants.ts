import type { PaymentMethod, SaleStatus } from '@/lib/api/types';

export const PAYMENT_METHODS: PaymentMethod[] = ['cash', 'pix', 'credit', 'debit'];

export const SALE_STATUSES: SaleStatus[] = ['Pending', 'Confirmed', 'Cancelled'];

export function saleActionErrorMessage(code: string): string {
  if (code === 'INSUFFICIENT_STOCK') {
    return 'Estoque insuficiente para confirmar esta venda.';
  }
  return 'Não foi possível concluir a operação.';
}

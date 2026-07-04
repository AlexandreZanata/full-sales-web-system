/** How a sale was paid — recorded only, no gateway capture (ADR-006). */
export enum PaymentMethod {
  Cash = 'Cash',
  Pix = 'Pix',
  Credit = 'Credit',
  Debit = 'Debit',
}

const PAYMENT_METHODS: readonly PaymentMethod[] = [
  PaymentMethod.Cash,
  PaymentMethod.Pix,
  PaymentMethod.Credit,
  PaymentMethod.Debit,
];

export function parsePaymentMethod(value: string): PaymentMethod {
  if ((PAYMENT_METHODS as readonly string[]).includes(value)) {
    return value as PaymentMethod;
  }
  throw new Error(`invalid payment method: ${value}`);
}

/** Format minor-unit amount (e.g. centavos) as currency. */
export function formatMoney(amount: number, currency = 'BRL', locale = 'pt-BR'): string {
  return new Intl.NumberFormat(locale, {
    style: 'currency',
    currency,
  }).format(amount / 100);
}

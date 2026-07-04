/** Format minor-unit amount (e.g. centavos) as currency. */
export function formatMoney(amount: number, currency = 'BRL'): string {
  return new Intl.NumberFormat('pt-BR', {
    style: 'currency',
    currency,
  }).format(amount / 100);
}

export function parsePriceInput(raw: string): number | null {
  const normalized = raw.replace(/\./g, '').replace(',', '.').trim();
  if (!normalized) return null;
  const value = Number.parseFloat(normalized);
  if (!Number.isFinite(value) || value < 0) return null;
  return Math.round(value * 100);
}

export function formatPriceInput(amount: number): string {
  return (amount / 100).toFixed(2).replace('.', ',');
}

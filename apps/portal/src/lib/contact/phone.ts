export function normalizeSalesContactPhone(raw: string): string {
  return raw.replace(/\D/g, '');
}

export function isValidSalesContactPhone(digits: string): boolean {
  return digits.length >= 10 && digits.length <= 15;
}

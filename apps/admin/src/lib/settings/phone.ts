export function normalizeSalesContactPhone(raw: string): string {
  return raw.replace(/\D/g, '');
}

export function isValidSalesContactPhone(digits: string): boolean {
  return digits.length >= 10 && digits.length <= 15;
}

export function validateSalesContactPhone(raw: string): string | undefined {
  const trimmed = raw.trim();
  if (!trimmed) {
    return undefined;
  }
  const digits = normalizeSalesContactPhone(trimmed);
  if (!isValidSalesContactPhone(digits)) {
    return 'settings.validation.salesContactPhoneInvalid';
  }
  return undefined;
}

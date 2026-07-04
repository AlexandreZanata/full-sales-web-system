const INVALID_CNPJ_SEQUENCES = new Set([
  '00000000000000',
  '11111111111111',
  '22222222222222',
  '33333333333333',
  '44444444444444',
  '55555555555555',
  '66666666666666',
  '77777777777777',
  '88888888888888',
  '99999999999999',
]);

function sumDigits(value: string, initialFactor: number, initialPosition = 0): number {
  let sum = 0;
  for (let factor = initialFactor; factor > 1; factor -= 1) {
    const index = initialPosition + (initialFactor - factor);
    sum += Number.parseInt(value.charAt(index), 10) * factor;
  }
  return sum;
}

function checkDigit(sum: number): string {
  const remainder = sum % 11;
  const digit = 11 - remainder;
  return (digit > 9 ? 0 : digit).toString();
}

/** Validates CNPJ check digits (RFB algorithm, aligned with br-validators). */
export function isValidCnpj(raw: string): boolean {
  const digits = raw.replace(/\D/g, '');
  if (digits.length !== 14 || INVALID_CNPJ_SEQUENCES.has(digits)) {
    return false;
  }
  const base = digits.slice(0, 12);
  const provided = digits.slice(12);
  const sum1 = sumDigits(base, 5) + sumDigits(base, 9, 4);
  const digit1 = checkDigit(sum1);
  const withFirst = base + digit1;
  const sum2 = sumDigits(withFirst, 6) + sumDigits(withFirst, 9, 5);
  const digit2 = checkDigit(sum2);
  return provided === `${digit1}${digit2}`;
}

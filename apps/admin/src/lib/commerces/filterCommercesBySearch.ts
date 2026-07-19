import type { CommerceSummary } from '@/lib/api/types';

export function formatCommerceOption(commerce: CommerceSummary): string {
  const name = commerce.tradeName || commerce.legalName;
  return commerce.cnpj ? `${name} · ${commerce.cnpj}` : name;
}

/** Match trade name, legal name, or CNPJ (case-insensitive substring). */
export function filterCommercesBySearch(
  commerces: CommerceSummary[],
  search: string,
): CommerceSummary[] {
  const normalized = search.trim().toLowerCase();
  if (!normalized) {
    return commerces;
  }
  const digits = normalized.replace(/\D/g, '');
  return commerces.filter((commerce) => {
    const trade = commerce.tradeName.toLowerCase();
    const legal = commerce.legalName.toLowerCase();
    const cnpj = commerce.cnpj.toLowerCase();
    const cnpjDigits = commerce.cnpj.replace(/\D/g, '');
    return (
      trade.includes(normalized) ||
      legal.includes(normalized) ||
      cnpj.includes(normalized) ||
      (digits.length > 0 && cnpjDigits.includes(digits))
    );
  });
}

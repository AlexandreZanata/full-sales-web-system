/** Prefer server displayCode for labels; fall back to truncated id. */
export function saleDisplayCode(sale: { displayCode?: string; id: string }): string {
  const code = sale.displayCode?.trim();
  if (code) {
    return code;
  }
  return `${sale.id.slice(0, 8)}…`;
}

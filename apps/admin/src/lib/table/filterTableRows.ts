const MAX_DEPTH = 3;

/** Flatten primitive values from a row for client-side table search. */
export function collectSearchText(value: unknown, depth = 0): string {
  if (value == null || depth > MAX_DEPTH) {
    return '';
  }
  if (typeof value === 'string' || typeof value === 'number' || typeof value === 'boolean') {
    return String(value).toLowerCase();
  }
  if (Array.isArray(value)) {
    return value.map((item) => collectSearchText(item, depth + 1)).join(' ');
  }
  if (typeof value === 'object') {
    return Object.values(value)
      .map((item) => collectSearchText(item, depth + 1))
      .join(' ');
  }
  return '';
}

export function rowMatchesSearch(row: unknown, search: string): boolean {
  const normalized = search.trim().toLowerCase();
  if (!normalized) {
    return true;
  }
  return collectSearchText(row).includes(normalized);
}

export function filterTableRows<T>(rows: T[], search: string, getSearchText?: (row: T) => string): T[] {
  const normalized = search.trim().toLowerCase();
  if (!normalized) {
    return rows;
  }
  if (getSearchText) {
    return rows.filter((row) => getSearchText(row).toLowerCase().includes(normalized));
  }
  return rows.filter((row) => rowMatchesSearch(row, normalized));
}

/** Catalog product layout — mirrors FoodKing `itemDesignEnum` (LIST | GRID). */
export type CatalogViewMode = 'list' | 'grid';

export const CATALOG_VIEW_MODES = ['list', 'grid'] as const satisfies readonly CatalogViewMode[];

export const DEFAULT_CATALOG_VIEW_MODE: CatalogViewMode = 'grid';

const CATALOG_VIEW_MODE_STORAGE_KEY = 'portal.catalog.viewMode';

export function isCatalogViewMode(value: string): value is CatalogViewMode {
  return value === 'list' || value === 'grid';
}

export function readCatalogViewMode(): CatalogViewMode {
  if (typeof window === 'undefined') {
    return DEFAULT_CATALOG_VIEW_MODE;
  }
  const stored = window.localStorage.getItem(CATALOG_VIEW_MODE_STORAGE_KEY);
  return stored && isCatalogViewMode(stored) ? stored : DEFAULT_CATALOG_VIEW_MODE;
}

export function writeCatalogViewMode(mode: CatalogViewMode): void {
  if (typeof window === 'undefined') {
    return;
  }
  window.localStorage.setItem(CATALOG_VIEW_MODE_STORAGE_KEY, mode);
}

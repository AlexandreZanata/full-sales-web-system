/** Catalog product layout — mirrors FoodKing `itemDesignEnum` (LIST | GRID). */
export type CatalogViewMode = 'list' | 'grid';

export const CATALOG_VIEW_MODES = ['list', 'grid'] as const satisfies readonly CatalogViewMode[];

export const DEFAULT_CATALOG_VIEW_MODE: CatalogViewMode = 'grid';

export function isCatalogViewMode(value: string): value is CatalogViewMode {
  return value === 'list' || value === 'grid';
}

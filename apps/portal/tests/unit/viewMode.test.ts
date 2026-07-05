import { describe, expect, it } from 'vitest';

import {
  CATALOG_VIEW_MODES,
  DEFAULT_CATALOG_VIEW_MODE,
  isCatalogViewMode,
} from '@/lib/catalog/viewMode';

describe('catalog view mode — Phase 45 contract', () => {
  it('defaults to grid view', () => {
    expect(DEFAULT_CATALOG_VIEW_MODE).toBe('grid');
  });

  it('recognizes list and grid modes', () => {
    expect(CATALOG_VIEW_MODES).toEqual(['list', 'grid']);
    expect(isCatalogViewMode('list')).toBe(true);
    expect(isCatalogViewMode('table')).toBe(false);
  });
});

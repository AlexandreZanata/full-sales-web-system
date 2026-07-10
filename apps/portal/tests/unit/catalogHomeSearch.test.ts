import { describe, expect, it } from 'vitest';

import { catalogHomeSearch } from '@/lib/catalog/catalogSearch';

describe('catalogHomeSearch — Phase 71J contract', () => {
  it('given_home_link_search_when_used_then_clears_menu_params', () => {
    expect(catalogHomeSearch).toEqual({});
    expect(catalogHomeSearch.category).toBeUndefined();
    expect(catalogHomeSearch.q).toBeUndefined();
  });
});

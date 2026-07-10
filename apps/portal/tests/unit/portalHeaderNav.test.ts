import { describe, expect, it } from 'vitest';

import {
  isPortalHomeActive,
  isPortalMenuActive,
  isPortalOffersActive,
} from '@/lib/catalog/portalHeaderNav';

describe('portalHeaderNav', () => {
  it('given_root_without_category_when_home_active_then_true', () => {
    expect(isPortalHomeActive('/', undefined)).toBe(true);
    expect(isPortalHomeActive('/', 'bebidas')).toBe(false);
  });

  it('given_category_or_product_path_when_menu_active_then_true', () => {
    expect(isPortalMenuActive('/', 'bebidas')).toBe(true);
    expect(isPortalMenuActive('/products/1', undefined)).toBe(true);
    expect(isPortalMenuActive('/', undefined)).toBe(false);
  });

  it('given_offers_hash_when_offers_active_then_true', () => {
    expect(isPortalOffersActive('#offers')).toBe(true);
    expect(isPortalOffersActive('#offer-banners')).toBe(true);
    expect(isPortalOffersActive('')).toBe(false);
  });
});

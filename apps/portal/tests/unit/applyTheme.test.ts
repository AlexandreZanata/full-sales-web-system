import { afterEach, describe, expect, it } from 'vitest';

import { FOODKING_BRAND_RED, applyThemePrimaryColor, hexToOklch } from '@/lib/settings/applyTheme';

describe('hexToOklch', () => {
  it('given_foodking_red_when_converted_then_matches_brand_oklch', () => {
    expect(hexToOklch('#FE1F00')).toBe('oklch(0.6339 0.2496 30.63)');
  });

  it('given_invalid_hex_when_converted_then_returns_null', () => {
    expect(hexToOklch('not-a-color')).toBeNull();
    expect(hexToOklch('#abc')).toBeNull();
  });
});

describe('applyThemePrimaryColor', () => {
  afterEach(() => {
    document.documentElement.style.removeProperty('--primary');
  });

  it('given_tenant_hex_when_applied_then_sets_primary_css_variable', () => {
    applyThemePrimaryColor('#FE1F00');
    expect(document.documentElement.style.getPropertyValue('--primary').trim()).toBe(
      'oklch(0.6339 0.2496 30.63)',
    );
  });

  it('given_missing_hex_when_applied_then_uses_foodking_default', () => {
    applyThemePrimaryColor(undefined);
    expect(document.documentElement.style.getPropertyValue('--primary').trim()).toBe(
      hexToOklch(FOODKING_BRAND_RED),
    );
  });
});

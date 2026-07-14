/**
 * Contract: seller TextSizePreset scales (Normal 1.0, Large 1.15, ExtraLarge 1.3)
 * and fromTag fallback — mirrors apps-mobile/seller shared a11y.
 */
import { describe, expect, it } from 'vitest';

import {
  effectiveFontScale,
  parseTextSizePreset,
  TEXT_SIZE_SCALE,
} from '@/lib/a11y/types';

describe('admin text size a11y — seller parity contract', () => {
  it('given_presets_when_read_then_match_seller_scale_factors', () => {
    expect(TEXT_SIZE_SCALE.Normal).toBe(1);
    expect(TEXT_SIZE_SCALE.Large).toBe(1.15);
    expect(TEXT_SIZE_SCALE.ExtraLarge).toBe(1.3);
  });

  it('given_system_scale_when_effective_then_multiplies_preset', () => {
    expect(effectiveFontScale(1, 'ExtraLarge')).toBe(1.3);
    expect(effectiveFontScale(1.5, 'Large')).toBeCloseTo(1.725, 3);
  });

  it('given_unknown_tag_when_parse_then_falls_back_to_normal', () => {
    expect(parseTextSizePreset('Large')).toBe('Large');
    expect(parseTextSizePreset('unknown')).toBe('Normal');
    expect(parseTextSizePreset(null)).toBe('Normal');
    expect(parseTextSizePreset(undefined)).toBe('Normal');
  });
});

/** Mirrors seller `TextSizePreset` scale factors (seller shared a11y contract). */
export const TEXT_SIZE_PRESETS = ['Normal', 'Large', 'ExtraLarge'] as const;

export type TextSizePreset = (typeof TEXT_SIZE_PRESETS)[number];

export const DEFAULT_TEXT_SIZE_PRESET: TextSizePreset = 'Normal';

export const TEXT_SIZE_SCALE: Record<TextSizePreset, number> = {
  Normal: 1,
  Large: 1.15,
  ExtraLarge: 1.3,
};

export function parseTextSizePreset(tag: string | null | undefined): TextSizePreset {
  if (tag === 'Normal' || tag === 'Large' || tag === 'ExtraLarge') {
    return tag;
  }
  return DEFAULT_TEXT_SIZE_PRESET;
}

export function effectiveFontScale(systemFontScale: number, preset: TextSizePreset): number {
  return systemFontScale * TEXT_SIZE_SCALE[preset];
}

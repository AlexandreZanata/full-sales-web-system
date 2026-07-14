import {
  DEFAULT_TEXT_SIZE_PRESET,
  effectiveFontScale,
  parseTextSizePreset,
  type TextSizePreset,
} from '@/lib/a11y/types';

const STORAGE_KEY = 'admin-text-size';

export function readStoredTextSizePreset(): TextSizePreset | null {
  if (typeof window === 'undefined') {
    return null;
  }
  const value = window.localStorage.getItem(STORAGE_KEY);
  if (value === null) {
    return null;
  }
  return parseTextSizePreset(value);
}

export function writeStoredTextSizePreset(preset: TextSizePreset): void {
  window.localStorage.setItem(STORAGE_KEY, preset);
}

export function resolveInitialTextSizePreset(): TextSizePreset {
  return readStoredTextSizePreset() ?? DEFAULT_TEXT_SIZE_PRESET;
}

export function applyTextSizePresetToDocument(preset: TextSizePreset): void {
  if (typeof document === 'undefined') {
    return;
  }
  // Browser zoom already reflects system preference; apply seller preset on top of 100%.
  const scalePercent = Math.round(effectiveFontScale(1, preset) * 100);
  document.documentElement.style.fontSize = `${scalePercent}%`;
  document.documentElement.dataset.textSize = preset;
}

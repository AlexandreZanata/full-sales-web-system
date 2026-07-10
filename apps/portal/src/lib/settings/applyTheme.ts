import { fetchSettings } from '@/lib/api/settings';

/** FoodKing / Zé Fominha brand red — default when tenant has no themePrimaryColor. */
export const FOODKING_BRAND_RED = '#FE1F00';

const HEX_PATTERN = /^#?([0-9a-f]{6})$/i;

function srgbToLinear(channel: number): number {
  const s = channel / 255;
  return s <= 0.04045 ? s / 12.92 : ((s + 0.055) / 1.055) ** 2.4;
}

/** Converts a 6-digit hex color to an oklch() string for CSS custom properties. */
export function hexToOklch(hex: string): string | null {
  const match = HEX_PATTERN.exec(hex.trim());
  if (!match) {
    return null;
  }

  const value = Number.parseInt(match[1], 16);
  const r = (value >> 16) & 255;
  const g = (value >> 8) & 255;
  const b = value & 255;

  const lr = srgbToLinear(r);
  const lg = srgbToLinear(g);
  const lb = srgbToLinear(b);

  const l = 0.4122214708 * lr + 0.5363325363 * lg + 0.0514459929 * lb;
  const m = 0.2119034982 * lr + 0.6806995451 * lg + 0.1073969566 * lb;
  const s = 0.0883024619 * lr + 0.2817188376 * lg + 0.6299787005 * lb;

  const lRoot = Math.cbrt(l);
  const mRoot = Math.cbrt(m);
  const sRoot = Math.cbrt(s);

  const lightness = 0.2104542553 * lRoot + 0.793617785 * mRoot - 0.0040720468 * sRoot;
  const a = 1.9779984951 * lRoot - 2.428592205 * mRoot + 0.4505937099 * sRoot;
  const bLab = 0.0259040371 * lRoot + 0.7827717662 * mRoot - 0.808675766 * sRoot;

  const chroma = Math.sqrt(a * a + bLab * bLab);
  let hue = (Math.atan2(bLab, a) * 180) / Math.PI;
  if (hue < 0) {
    hue += 360;
  }

  return `oklch(${lightness.toFixed(4)} ${chroma.toFixed(4)} ${hue.toFixed(2)})`;
}

/** Applies tenant brand color to `--primary`; falls back to FoodKing red when invalid. */
export function applyThemePrimaryColor(hex: string | null | undefined): void {
  const candidate = hex?.trim() || FOODKING_BRAND_RED;
  const oklch = hexToOklch(candidate) ?? hexToOklch(FOODKING_BRAND_RED);
  if (oklch) {
    document.documentElement.style.setProperty('--primary', oklch);
  }
}

/** Sets default tokens immediately, then overrides from public/authenticated settings. */
export async function bootstrapPortalTheme(): Promise<void> {
  applyThemePrimaryColor(FOODKING_BRAND_RED);

  try {
    const settings = await fetchSettings();
    applyThemePrimaryColor(settings.themePrimaryColor);
  } catch {
    // Keep default brand red when settings are unavailable offline or during boot.
  }
}

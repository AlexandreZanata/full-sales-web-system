import { DEFAULT_LOCALE, type Locale } from '@/lib/i18n/types';

const STORAGE_KEY = 'portal-locale';

export function readStoredLocale(): Locale | null {
  if (typeof window === 'undefined') {
    return null;
  }
  const value = window.localStorage.getItem(STORAGE_KEY);
  if (value === 'en' || value === 'pt-BR') {
    return value;
  }
  return null;
}

export function writeStoredLocale(locale: Locale): void {
  window.localStorage.setItem(STORAGE_KEY, locale);
}

/**
 * Map browser / OS language to a supported portal locale.
 * Unrecognized → Portuguese (never default to English).
 */
export function localeFromBrowserLanguage(language: string | undefined | null): Locale {
  const normalized = language?.trim().toLowerCase() ?? '';
  if (normalized.startsWith('en')) {
    return 'en';
  }
  if (normalized.startsWith('pt')) {
    return 'pt-BR';
  }
  return DEFAULT_LOCALE;
}

export function resolveInitialLocale(): Locale {
  const stored = readStoredLocale();
  if (stored) {
    return stored;
  }
  if (typeof navigator === 'undefined') {
    return DEFAULT_LOCALE;
  }
  return localeFromBrowserLanguage(navigator.language);
}

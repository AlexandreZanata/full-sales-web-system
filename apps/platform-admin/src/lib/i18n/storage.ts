const STORAGE_KEY = 'platform-admin-locale';

import { DEFAULT_LOCALE, type Locale } from '@/lib/i18n/types';

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

export function resolveInitialLocale(): Locale {
  return readStoredLocale() ?? DEFAULT_LOCALE;
}

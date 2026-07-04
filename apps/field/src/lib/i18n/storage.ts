import { DEFAULT_LOCALE, type Locale } from '@/lib/i18n/types';

const STORAGE_KEY = 'field-locale';

export function resolveInitialLocale(): Locale {
  if (typeof window === 'undefined') return DEFAULT_LOCALE;
  const value = window.localStorage.getItem(STORAGE_KEY);
  if (value === 'en' || value === 'pt-BR') return value;
  return DEFAULT_LOCALE;
}

export function writeStoredLocale(locale: Locale): void {
  window.localStorage.setItem(STORAGE_KEY, locale);
}

export type Locale = 'en' | 'pt-BR';

export const LOCALES: Locale[] = ['en', 'pt-BR'];

export const DEFAULT_LOCALE: Locale = 'en';

export const LOCALE_LABELS: Record<Locale, string> = {
  en: 'English',
  'pt-BR': 'Português (BR)',
};

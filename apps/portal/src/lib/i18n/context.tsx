import { createContext, useCallback, useContext, useMemo, useState, type ReactNode } from 'react';

import type { MessageKey, Messages } from '@/lib/i18n/messages';
import { en } from '@/lib/i18n/locales/en';
import { ptBR } from '@/lib/i18n/locales/pt-BR';
import { resolveInitialLocale, writeStoredLocale } from '@/lib/i18n/storage';
import { orderStatusLabel, translate } from '@/lib/i18n/translate';
import { LOCALE_LABELS, type Locale } from '@/lib/i18n/types';

type I18nContextValue = {
  locale: Locale;
  setLocale: (locale: Locale) => void;
  t: (key: MessageKey) => string;
  orderStatus: (status: string) => string;
};

const catalogs: Record<Locale, Messages> = {
  en,
  'pt-BR': ptBR,
};

const I18nContext = createContext<I18nContextValue | null>(null);

export function I18nProvider({
  children,
  initialLocale,
}: {
  children: ReactNode;
  initialLocale?: Locale;
}) {
  const [locale, setLocaleState] = useState<Locale>(() => initialLocale ?? resolveInitialLocale());

  const setLocale = useCallback((next: Locale) => {
    writeStoredLocale(next);
    setLocaleState(next);
  }, []);

  const value = useMemo<I18nContextValue>(() => {
    const messages = catalogs[locale];
    return {
      locale,
      setLocale,
      t: (key) => translate(messages, key),
      orderStatus: (status) => orderStatusLabel(messages, status),
    };
  }, [locale, setLocale]);

  return <I18nContext.Provider value={value}>{children}</I18nContext.Provider>;
}

export function useI18n(): I18nContextValue {
  const context = useContext(I18nContext);
  if (!context) {
    throw new Error('useI18n must be used within I18nProvider');
  }
  return context;
}

export { LOCALE_LABELS };

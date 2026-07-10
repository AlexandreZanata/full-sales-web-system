import { LOCALE_LABELS, useI18n } from '@/lib/i18n/context';
import type { Locale } from '@/lib/i18n/types';
import { cn } from '@/lib/utils';

const LOCALE_SHORT: Record<Locale, string> = {
  en: 'EN',
  'pt-BR': 'PT',
};

type LocaleSwitcherProps = {
  variant?: 'default' | 'pill';
};

export function LocaleSwitcher({ variant = 'default' }: LocaleSwitcherProps) {
  const { locale, setLocale, t } = useI18n();

  if (variant === 'pill') {
    return (
      <label className="portal-locale-pill">
        <span className="sr-only">{t('shell.locale')}</span>
        <select
          value={locale}
          aria-label={t('shell.locale')}
          className="bg-transparent text-sm font-semibold capitalize text-foreground outline-none"
          onChange={(event) => {
            setLocale(event.target.value as Locale);
          }}
        >
          {(Object.keys(LOCALE_LABELS) as Locale[]).map((key) => (
            <option key={key} value={key}>
              {LOCALE_SHORT[key]}
            </option>
          ))}
        </select>
      </label>
    );
  }

  return (
    <label className="inline-flex items-center gap-2 text-xs text-muted-foreground">
      <span className="sr-only">{t('shell.locale')}</span>
      <select
        value={locale}
        aria-label={t('shell.locale')}
        className={cn('h-8 rounded-md border border-input bg-surface px-2 text-xs text-foreground')}
        onChange={(event) => {
          setLocale(event.target.value as Locale);
        }}
      >
        {(Object.keys(LOCALE_LABELS) as Locale[]).map((key) => (
          <option key={key} value={key}>
            {LOCALE_LABELS[key]}
          </option>
        ))}
      </select>
    </label>
  );
}

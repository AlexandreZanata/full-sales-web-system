import { LOCALE_LABELS, useI18n } from '@/lib/i18n/context';
import type { Locale } from '@/lib/i18n/types';

export function LocaleSwitcher() {
  const { locale, setLocale, t } = useI18n();

  return (
    <label className="inline-flex items-center gap-2 text-xs text-muted-foreground">
      <span className="sr-only">{t('shell.locale')}</span>
      <select
        value={locale}
        aria-label={t('shell.locale')}
        className="h-8 rounded-md border border-input bg-surface px-2 text-xs text-foreground"
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

import { useI18n, LOCALE_LABELS } from '@/lib/i18n/context';
import { LOCALES, type Locale } from '@/lib/i18n/types';
import { cn } from '@/lib/utils';

type LocaleSwitcherProps = {
  className?: string;
};

export function LocaleSwitcher({ className }: LocaleSwitcherProps) {
  const { locale, setLocale, t } = useI18n();

  return (
    <label
      className={cn('inline-flex items-center gap-2 text-sm text-muted-foreground', className)}
    >
      <span className="sr-only">{t('shell.locale')}</span>
      <select
        aria-label={t('shell.locale')}
        value={locale}
        className="h-10 min-h-10 rounded-md border border-hairline bg-surface px-2 text-sm text-foreground"
        onChange={(event) => {
          setLocale(event.target.value as Locale);
        }}
      >
        {LOCALES.map((value) => (
          <option key={value} value={value}>
            {LOCALE_LABELS[value]}
          </option>
        ))}
      </select>
    </label>
  );
}

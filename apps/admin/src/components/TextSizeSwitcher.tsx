import { useAccessibility } from '@/lib/a11y/context';
import { TEXT_SIZE_PRESETS, type TextSizePreset } from '@/lib/a11y/types';
import { useI18n } from '@/lib/i18n/context';
import type { MessageKey } from '@/lib/i18n/messages';
import { cn } from '@/lib/utils';

const PRESET_LABEL_KEY: Record<TextSizePreset, MessageKey> = {
  Normal: 'a11y.textSizeNormal',
  Large: 'a11y.textSizeLarge',
  ExtraLarge: 'a11y.textSizeExtraLarge',
};

type TextSizeSwitcherProps = {
  className?: string;
  variant?: 'select' | 'chips';
};

export function TextSizeSwitcher({ className, variant = 'select' }: TextSizeSwitcherProps) {
  const { t } = useI18n();
  const { textSizePreset, setTextSizePreset } = useAccessibility();

  if (variant === 'chips') {
    return (
      <div
        role="group"
        aria-label={t('a11y.textSizeLabel')}
        className={cn('flex flex-wrap gap-2', className)}
      >
        {TEXT_SIZE_PRESETS.map((preset) => {
          const selected = textSizePreset === preset;
          return (
            <button
              key={preset}
              type="button"
              aria-pressed={selected}
              className={cn(
                'inline-flex h-10 min-h-10 items-center rounded-md border px-3 text-sm font-medium transition-colors',
                selected
                  ? 'border-foreground bg-foreground text-background'
                  : 'border-hairline bg-surface text-foreground hover:bg-surface-muted',
              )}
              onClick={() => {
                setTextSizePreset(preset);
              }}
            >
              {t(PRESET_LABEL_KEY[preset])}
            </button>
          );
        })}
      </div>
    );
  }

  return (
    <label
      className={cn('inline-flex items-center gap-2 text-sm text-muted-foreground', className)}
    >
      <span className="sr-only">{t('a11y.textSizeLabel')}</span>
      <select
        aria-label={t('a11y.textSizeLabel')}
        value={textSizePreset}
        className="h-10 min-h-10 rounded-md border border-hairline bg-surface px-2 text-sm text-foreground"
        onChange={(event) => {
          setTextSizePreset(event.target.value as TextSizePreset);
        }}
      >
        {TEXT_SIZE_PRESETS.map((preset) => (
          <option key={preset} value={preset}>
            {t(PRESET_LABEL_KEY[preset])}
          </option>
        ))}
      </select>
    </label>
  );
}

import { LocaleSwitcher } from '@/components/LocaleSwitcher';
import { TextSizeSwitcher } from '@/components/TextSizeSwitcher';
import { useI18n } from '@/lib/i18n/context';
import { cn } from '@/lib/utils';

type AccessibilityControlsProps = {
  className?: string;
  /** `panel` matches seller login a11y panel; `inline` matches admin top bar. */
  layout?: 'panel' | 'inline';
};

export function AccessibilityControls({
  className,
  layout = 'inline',
}: AccessibilityControlsProps) {
  const { t } = useI18n();

  if (layout === 'panel') {
    return (
      <div className={cn('space-y-4', className)}>
        <div className="space-y-2">
          <p className="text-xs font-semibold uppercase tracking-[0.12em] text-muted-foreground">
            {t('a11y.language')}
          </p>
          <LocaleSwitcher className="w-full [&_select]:w-full" />
        </div>
        <div className="space-y-2">
          <p className="text-xs font-semibold uppercase tracking-[0.12em] text-muted-foreground">
            {t('a11y.textSizeLabel')}
          </p>
          <TextSizeSwitcher variant="chips" />
        </div>
      </div>
    );
  }

  return (
    <div className={cn('flex flex-wrap items-center gap-2', className)}>
      <LocaleSwitcher />
      <TextSizeSwitcher />
    </div>
  );
}

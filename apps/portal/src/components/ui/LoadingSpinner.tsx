import { useI18n } from '@/lib/i18n/context';
import { cn } from '@/lib/utils';

type LoadingSpinnerProps = {
  className?: string;
  label?: string;
};

export function LoadingSpinner({ className, label }: LoadingSpinnerProps) {
  const { t } = useI18n();

  return (
    <div
      className={cn(
        'flex items-center justify-center gap-2 text-sm text-muted-foreground',
        className,
      )}
    >
      <span
        className="size-4 animate-spin rounded-full border-2 border-hairline border-t-foreground"
        aria-hidden
      />
      <span>{label ?? t('common.loading.default')}</span>
    </div>
  );
}

import { useI18n } from '@/lib/i18n/context';
import { cn } from '@/lib/utils';

type ActiveBadgeProps = {
  active: boolean;
};

export function ActiveBadge({ active }: ActiveBadgeProps) {
  const { t } = useI18n();

  return (
    <span
      className={cn(
        'inline-flex rounded-full border px-2.5 py-0.5 text-xs font-medium',
        active
          ? 'border-status-active/30 text-status-active'
          : 'border-hairline text-muted-foreground',
      )}
    >
      {active ? t('common.active.active') : t('common.active.inactive')}
    </span>
  );
}

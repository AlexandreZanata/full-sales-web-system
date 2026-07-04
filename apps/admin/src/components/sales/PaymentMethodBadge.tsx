import { useI18n } from '@/lib/i18n/context';
import { translatePaymentMethod } from '@/lib/i18n/labels';
import { cn } from '@/lib/utils';

type PaymentMethodBadgeProps = {
  method: string;
  className?: string;
};

export function PaymentMethodBadge({ method, className }: PaymentMethodBadgeProps) {
  const { t } = useI18n();

  return (
    <span
      className={cn(
        'inline-flex items-center rounded-full border border-hairline bg-surface-muted px-2.5 py-0.5 text-xs font-medium text-foreground',
        className,
      )}
    >
      {translatePaymentMethod(t, method)}
    </span>
  );
}

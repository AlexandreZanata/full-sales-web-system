import { useI18n } from '@/lib/i18n/context';
import { translateDeclaredPayment } from '@/lib/i18n/labels';
import { isDeclaredPayment } from '@/lib/sales/constants';
import { cn } from '@/lib/utils';

type DeclaredPaymentBadgeProps = {
  method: string;
  received: boolean;
  className?: string;
};

export function DeclaredPaymentBadge({ method, received, className }: DeclaredPaymentBadgeProps) {
  const { t } = useI18n();
  const declared = isDeclaredPayment(method, received);

  return (
    <span
      className={cn(
        'inline-flex items-center rounded-full border px-2.5 py-0.5 text-xs font-medium',
        declared
          ? received
            ? 'border-status-active/30 text-status-active'
            : 'border-status-warning/30 text-status-warning'
          : 'border-hairline text-muted-foreground',
        className,
      )}
    >
      {translateDeclaredPayment(t, method, received)}
    </span>
  );
}

import { useI18n } from '@/lib/i18n/context';
import { ORDER_TIMELINE, orderStatusIndex } from '@/lib/orders/constants';
import { cn } from '@/lib/utils';

type OrderStatusTimelineProps = {
  status: string;
};

export function OrderStatusTimeline({ status }: OrderStatusTimelineProps) {
  const { orderStatus, t } = useI18n();
  const currentIndex = orderStatusIndex(status);
  const isRejected = status === 'Rejected' || status === 'Cancelled';

  return (
    <div className="space-y-3">
      <h3 className="text-sm font-semibold text-foreground">{t('orders.timeline')}</h3>
      {isRejected ? (
        <p className="text-sm text-destructive">{orderStatus(status)}</p>
      ) : (
        <ol className="space-y-2">
          {ORDER_TIMELINE.map((step, index) => {
            const done = currentIndex >= index;
            const active = currentIndex === index;
            return (
              <li key={step} className="flex items-center gap-3 text-sm">
                <span
                  className={cn(
                    'flex size-6 shrink-0 items-center justify-center rounded-full border text-xs font-semibold',
                    done
                      ? 'border-primary bg-primary text-primary-foreground'
                      : 'border-hairline text-muted-foreground',
                    active && 'ring-2 ring-accent ring-offset-2',
                  )}
                >
                  {index + 1}
                </span>
                <span className={done ? 'text-foreground' : 'text-muted-foreground'}>
                  {orderStatus(step)}
                </span>
              </li>
            );
          })}
        </ol>
      )}
    </div>
  );
}

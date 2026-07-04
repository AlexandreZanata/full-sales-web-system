import { DomainStatusBadge } from '@/components/status/DomainStatusBadge';
import { Card } from '@/components/ui/Card';
import {
  deliveryStatusColors,
  getDeliveryStatusToken,
  type DeliveryStatus,
} from '@/lib/admin-tokens';
import { cn } from '@/lib/utils';

const TIMELINE_STEPS: DeliveryStatus[] = ['Waiting', 'InTransit', 'Delivered'];

type DeliveryStatusTimelineProps = {
  status: string;
};

function stepIndex(status: DeliveryStatus): number {
  if (status === 'Failed') {
    return 2;
  }
  return TIMELINE_STEPS.indexOf(status);
}

export function DeliveryStatusTimeline({ status }: DeliveryStatusTimelineProps) {
  const current = status as DeliveryStatus;
  const activeIndex = stepIndex(current);
  const isFailed = current === 'Failed';

  return (
    <Card className="space-y-4 p-5">
      <h3 className="text-sm font-semibold text-foreground">Status timeline</h3>
      <ol className="space-y-3">
        {TIMELINE_STEPS.map((step, index) => {
          const completed = !isFailed && index < activeIndex;
          const active = !isFailed && index === activeIndex;
          const failedTerminal = isFailed && index === TIMELINE_STEPS.length - 1;
          const token = failedTerminal ? deliveryStatusColors.Failed : getDeliveryStatusToken(step);

          return (
            <li key={step} className="flex items-center gap-3">
              <span
                className={cn(
                  'flex size-6 shrink-0 items-center justify-center rounded-full border text-xs font-semibold',
                  completed || active
                    ? 'border-foreground bg-foreground text-background'
                    : 'border-hairline bg-surface text-muted-foreground',
                )}
                aria-hidden
              >
                {completed ? '✓' : index + 1}
              </span>
              <div className="flex flex-1 items-center justify-between gap-2">
                <span className="text-sm text-foreground">{token.label}</span>
                {(active || failedTerminal) && <DomainStatusBadge colors={token} />}
              </div>
            </li>
          );
        })}
      </ol>
      {isFailed ? (
        <p className="text-sm text-destructive">Delivery marked as failed before completion.</p>
      ) : null}
    </Card>
  );
}

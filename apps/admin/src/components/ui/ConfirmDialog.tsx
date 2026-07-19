import { AlertTriangle, CircleCheck } from 'lucide-react';

import { Button } from '@/components/ui/Button';
import { useI18n } from '@/lib/i18n/context';
import { cn } from '@/lib/utils';

type ConfirmDialogProps = {
  open: boolean;
  title: string;
  message: string;
  confirmLabel?: string;
  onConfirm: () => void;
  onCancel: () => void;
  destructive?: boolean;
  isLoading?: boolean;
};

export function ConfirmDialog({
  open,
  title,
  message,
  confirmLabel,
  onConfirm,
  onCancel,
  destructive = false,
  isLoading = false,
}: ConfirmDialogProps) {
  const { t } = useI18n();
  const resolvedConfirmLabel = confirmLabel ?? t('common.confirm');
  const Icon = destructive ? AlertTriangle : CircleCheck;

  if (!open) return null;

  return (
    <div
      className="fixed inset-0 z-50 flex items-center justify-center bg-foreground/45 p-4 backdrop-blur-[2px]"
      role="presentation"
      onClick={onCancel}
    >
      <div
        className="w-full max-w-sm rounded-2xl border border-hairline bg-surface p-5 shadow-[0_24px_64px_rgba(26,26,26,0.18)]"
        role="dialog"
        aria-modal="true"
        aria-labelledby="confirm-title"
        aria-describedby="confirm-message"
        onClick={(event) => event.stopPropagation()}
      >
        <div className="flex gap-3">
          <span
            className={cn(
              'flex size-10 shrink-0 items-center justify-center rounded-full',
              destructive ? 'bg-destructive/10 text-destructive' : 'bg-status-active/10 text-status-active',
            )}
            aria-hidden
          >
            <Icon className="size-5" />
          </span>
          <div className="min-w-0 space-y-1.5 pt-0.5">
            <h2 id="confirm-title" className="text-base font-semibold leading-snug text-foreground">
              {title}
            </h2>
            <p id="confirm-message" className="text-sm leading-relaxed text-muted-foreground">
              {message}
            </p>
          </div>
        </div>
        <div className="mt-5 flex justify-end gap-2 border-t border-hairline pt-4">
          <Button variant="secondary" onClick={onCancel} disabled={isLoading}>
            {t('common.cancel')}
          </Button>
          <Button
            variant={destructive ? 'danger' : 'success'}
            onClick={onConfirm}
            disabled={isLoading}
          >
            {isLoading ? t('common.working') : resolvedConfirmLabel}
          </Button>
        </div>
      </div>
    </div>
  );
}

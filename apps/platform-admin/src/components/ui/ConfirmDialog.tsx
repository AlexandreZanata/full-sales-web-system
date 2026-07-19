import { createPortal } from 'react-dom';
import type { ReactNode } from 'react';

import { Button } from '@/components/ui/Button';
import { Card } from '@/components/ui/Card';
import { useI18n } from '@/lib/i18n/context';

type ConfirmDialogProps = {
  open: boolean;
  title: string;
  message: string;
  confirmLabel?: string;
  onConfirm: () => void;
  onCancel: () => void;
  destructive?: boolean;
  isLoading?: boolean;
  confirmDisabled?: boolean;
  children?: ReactNode;
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
  confirmDisabled = false,
  children,
}: ConfirmDialogProps) {
  const { t } = useI18n();
  const resolvedConfirmLabel = confirmLabel ?? t('common.confirm');

  if (!open || typeof document === 'undefined') {
    return null;
  }

  return createPortal(
    <div
      className="fixed inset-0 z-[100] flex min-h-dvh items-center justify-center bg-black/55 p-4"
      role="presentation"
      onClick={onCancel}
    >
      <Card
        className="relative z-[101] w-full max-w-md p-6 shadow-xl"
        role="dialog"
        aria-modal="true"
        aria-labelledby="confirm-title"
        onClick={(event) => {
          event.stopPropagation();
        }}
      >
        <h2 id="confirm-title" className="text-lg font-semibold text-foreground">
          {title}
        </h2>
        <p className="mt-2 text-sm text-muted-foreground">{message}</p>
        {children ? <div className="mt-4">{children}</div> : null}
        <div className="mt-6 flex justify-end gap-2">
          <Button variant="secondary" onClick={onCancel} disabled={isLoading}>
            {t('common.cancel')}
          </Button>
          <Button
            variant={destructive ? 'danger' : 'primary'}
            onClick={onConfirm}
            disabled={isLoading || confirmDisabled}
          >
            {isLoading ? t('common.working') : resolvedConfirmLabel}
          </Button>
        </div>
      </Card>
    </div>,
    document.body,
  );
}

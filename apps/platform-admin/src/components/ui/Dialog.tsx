import { createPortal } from 'react-dom';
import type { ReactNode } from 'react';

import { Button } from '@/components/ui/Button';
import { Card } from '@/components/ui/Card';
import { useI18n } from '@/lib/i18n/context';
import { cn } from '@/lib/utils';

type DialogProps = {
  open: boolean;
  title: string;
  onClose: () => void;
  children: ReactNode;
  footer?: ReactNode;
  className?: string;
};

export function Dialog({ open, title, onClose, children, footer, className }: DialogProps) {
  const { t } = useI18n();

  if (!open || typeof document === 'undefined') {
    return null;
  }

  return createPortal(
    <div
      className="fixed inset-0 z-[100] flex min-h-dvh items-center justify-center bg-black/55 p-4"
      role="presentation"
      onClick={onClose}
    >
      <Card
        className={cn(
          'relative z-[101] flex max-h-[90dvh] w-full max-w-lg flex-col overflow-hidden p-0 shadow-xl',
          className,
        )}
        role="dialog"
        aria-modal="true"
        aria-labelledby="dialog-title"
        onClick={(event) => {
          event.stopPropagation();
        }}
      >
        <div className="flex items-start justify-between gap-3 border-b border-hairline px-5 py-4">
          <h2 id="dialog-title" className="text-lg font-semibold text-foreground">
            {title}
          </h2>
          <Button type="button" variant="secondary" onClick={onClose}>
            {t('common.close')}
          </Button>
        </div>
        <div className="overflow-y-auto px-5 py-4">{children}</div>
        {footer ? (
          <div className="flex flex-wrap justify-end gap-2 border-t border-hairline px-5 py-4">
            {footer}
          </div>
        ) : null}
      </Card>
    </div>,
    document.body,
  );
}

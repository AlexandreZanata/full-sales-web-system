import { Button } from '@/components/ui/Button';
import { Card } from '@/components/ui/Card';

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
  confirmLabel = 'Confirm',
  onConfirm,
  onCancel,
  destructive = false,
  isLoading = false,
}: ConfirmDialogProps) {
  if (!open) return null;

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-foreground/40 p-4">
      <Card
        className="flex aspect-square w-full max-w-md flex-col justify-between p-6"
        role="dialog"
        aria-modal="true"
        aria-labelledby="confirm-title"
      >
        <div className="min-h-0 flex-1 overflow-y-auto">
          <h2 id="confirm-title" className="text-lg font-semibold text-foreground">
            {title}
          </h2>
          <p className="mt-2 text-sm text-muted-foreground">{message}</p>
        </div>
        <div className="mt-6 flex shrink-0 justify-end gap-2">
          <Button variant="secondary" onClick={onCancel} disabled={isLoading}>
            Cancel
          </Button>
          <Button
            variant={destructive ? 'danger' : 'primary'}
            onClick={onConfirm}
            disabled={isLoading}
          >
            {isLoading ? 'Working…' : confirmLabel}
          </Button>
        </div>
      </Card>
    </div>
  );
}

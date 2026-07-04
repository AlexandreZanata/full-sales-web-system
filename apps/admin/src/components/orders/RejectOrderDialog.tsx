import { useState } from 'react';

import { Button } from '@/components/ui/Button';
import { Card } from '@/components/ui/Card';
import { Textarea } from '@/components/ui/Textarea';
import { useI18n } from '@/lib/i18n/context';
import { translateFormError } from '@/lib/i18n/labels';
import {
  hasFormErrors,
  validateRejectOrderForm,
  type RejectOrderFormValues,
} from '@/lib/orders/validation';

type RejectOrderDialogProps = {
  open: boolean;
  isLoading?: boolean;
  onCancel: () => void;
  onConfirm: (reason: string) => void;
};

export function RejectOrderDialog({
  open,
  isLoading = false,
  onCancel,
  onConfirm,
}: RejectOrderDialogProps) {
  const { t } = useI18n();
  const [values, setValues] = useState<RejectOrderFormValues>({ reason: '' });
  const [errors, setErrors] = useState<Partial<Record<keyof RejectOrderFormValues, string>>>({});

  if (!open) return null;

  function handleConfirm() {
    const nextErrors = validateRejectOrderForm(values);
    setErrors(nextErrors);
    if (hasFormErrors(nextErrors)) {
      return;
    }
    onConfirm(values.reason.trim());
  }

  function handleCancel() {
    setValues({ reason: '' });
    setErrors({});
    onCancel();
  }

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-foreground/40 p-4">
      <Card
        className="flex w-full max-w-md flex-col p-6"
        role="dialog"
        aria-modal="true"
        aria-labelledby="reject-order-title"
      >
        <h2 id="reject-order-title" className="text-lg font-semibold text-foreground">
          {t('orders.rejectDialog.title')}
        </h2>
        <p className="mt-2 text-sm text-muted-foreground">{t('orders.rejectDialog.description')}</p>
        <div className="mt-4">
          <Textarea
            label={t('forms.fields.rejectionReason')}
            value={values.reason}
            onChange={(event) => {
              setValues({ reason: event.target.value });
              if (errors.reason) {
                setErrors({});
              }
            }}
            error={translateFormError(t, errors.reason)}
            disabled={isLoading}
            rows={4}
          />
        </div>
        <div className="mt-6 flex justify-end gap-2">
          <Button variant="secondary" onClick={handleCancel} disabled={isLoading}>
            {t('common.cancel')}
          </Button>
          <Button variant="danger" onClick={handleConfirm} disabled={isLoading}>
            {isLoading ? t('orders.rejectDialog.submitting') : t('orders.rejectDialog.submit')}
          </Button>
        </div>
      </Card>
    </div>
  );
}

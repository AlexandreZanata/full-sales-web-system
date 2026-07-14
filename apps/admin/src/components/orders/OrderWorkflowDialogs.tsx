import { AssignDeliveryDialog } from '@/components/orders/AssignDeliveryDialog';
import { RejectOrderDialog } from '@/components/orders/RejectOrderDialog';
import { ConfirmDialog } from '@/components/ui/ConfirmDialog';
import { useToast } from '@/hooks/useToast';
import { ApiError } from '@/lib/api/client';
import { assignDelivery, cancelOrder, rejectOrder } from '@/lib/api/orders';
import type { User } from '@/lib/api/types';
import { useI18n } from '@/lib/i18n/context';
import { orderActionErrorKey } from '@/lib/i18n/labels';

type OrderWorkflowDialogsProps = {
  orderId: string;
  rejectOpen: boolean;
  assignOpen: boolean;
  cancelOpen: boolean;
  drivers: User[];
  actionLoading: boolean;
  onRejectOpenChange: (open: boolean) => void;
  onAssignOpenChange: (open: boolean) => void;
  onCancelOpenChange: (open: boolean) => void;
  onActionLoadingChange: (loading: boolean) => void;
  onInvalidate: () => Promise<void>;
};

export function OrderWorkflowDialogs({
  orderId,
  rejectOpen,
  assignOpen,
  cancelOpen,
  drivers,
  actionLoading,
  onRejectOpenChange,
  onAssignOpenChange,
  onCancelOpenChange,
  onActionLoadingChange,
  onInvalidate,
}: OrderWorkflowDialogsProps) {
  const { t } = useI18n();
  const toast = useToast();

  async function runGuarded(
    action: () => Promise<unknown>,
    successMessage: string,
    close: () => void,
  ) {
    onActionLoadingChange(true);
    try {
      await action();
      await onInvalidate();
      toast.success(successMessage);
      close();
    } catch (error) {
      const message =
        error instanceof ApiError ? t(orderActionErrorKey(error.code)) : t('errors.actionFailed');
      toast.error(message);
    } finally {
      onActionLoadingChange(false);
    }
  }

  return (
    <>
      <RejectOrderDialog
        open={rejectOpen}
        isLoading={actionLoading}
        onCancel={() => {
          onRejectOpenChange(false);
        }}
        onConfirm={(reason) => {
          void runGuarded(
            () => rejectOrder(orderId, reason),
            t('orders.toast.rejected'),
            () => {
              onRejectOpenChange(false);
            },
          );
        }}
      />
      <AssignDeliveryDialog
        open={assignOpen}
        drivers={drivers}
        isLoading={actionLoading}
        onCancel={() => {
          onAssignOpenChange(false);
        }}
        onConfirm={(driverId) => {
          void runGuarded(
            () => assignDelivery(orderId, driverId),
            t('orders.toast.deliveryAssigned'),
            () => {
              onAssignOpenChange(false);
            },
          );
        }}
      />
      <ConfirmDialog
        open={cancelOpen}
        title={t('orders.cancelDialog.title')}
        message={t('orders.cancelDialog.message')}
        confirmLabel={t('orders.cancelDialog.confirm')}
        destructive
        isLoading={actionLoading}
        onCancel={() => {
          onCancelOpenChange(false);
        }}
        onConfirm={() => {
          void runGuarded(
            () => cancelOrder(orderId),
            t('orders.toast.cancelled'),
            () => {
              onCancelOpenChange(false);
            },
          );
        }}
      />
    </>
  );
}

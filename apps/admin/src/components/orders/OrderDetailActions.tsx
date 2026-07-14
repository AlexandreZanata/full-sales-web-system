import { Button } from '@/components/ui/Button';
import { useI18n } from '@/lib/i18n/context';

type OrderDetailActionsProps = {
  status: string;
  hasDelivery: boolean;
  actionLoading: boolean;
  onApprove: () => void;
  onReject: () => void;
  onStartPicking: () => void;
  onAssignDelivery: () => void;
  onCancel: () => void;
};

export function OrderDetailActions({
  status,
  hasDelivery,
  actionLoading,
  onApprove,
  onReject,
  onStartPicking,
  onAssignDelivery,
  onCancel,
}: OrderDetailActionsProps) {
  const { t } = useI18n();
  const canApprove = status === 'PendingApproval';
  const canReject = status === 'PendingApproval';
  const canStartPicking = status === 'Approved';
  const canAssignDelivery = (status === 'Approved' || status === 'Picking') && !hasDelivery;
  const canCancel = status === 'Approved' || status === 'Picking';

  return (
    <div className="flex flex-wrap gap-2">
      {canApprove ? (
        <Button variant="success" disabled={actionLoading} onClick={onApprove}>
          {t('orders.detail.actions.approve')}
        </Button>
      ) : null}
      {canReject ? (
        <Button variant="danger" disabled={actionLoading} onClick={onReject}>
          {t('orders.detail.actions.reject')}
        </Button>
      ) : null}
      {canStartPicking ? (
        <Button variant="secondary" disabled={actionLoading} onClick={onStartPicking}>
          {t('orders.detail.actions.startPicking')}
        </Button>
      ) : null}
      {canAssignDelivery ? (
        <Button variant="secondary" disabled={actionLoading} onClick={onAssignDelivery}>
          {t('orders.detail.actions.assignDelivery')}
        </Button>
      ) : null}
      {canCancel ? (
        <Button variant="danger" disabled={actionLoading} onClick={onCancel}>
          {t('orders.detail.actions.cancel')}
        </Button>
      ) : null}
    </div>
  );
}

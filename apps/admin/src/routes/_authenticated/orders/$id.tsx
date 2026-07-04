import { Link, createFileRoute } from '@tanstack/react-router';
import { useQuery, useQueryClient } from '@tanstack/react-query';
import { useMemo, useState, type ReactNode } from 'react';

import { AssignDeliveryDialog } from '@/components/orders/AssignDeliveryDialog';
import { OrderStatusBadge } from '@/components/orders/OrderStatusBadge';
import { RejectOrderDialog } from '@/components/orders/RejectOrderDialog';
import { DomainStatusBadge } from '@/components/status/DomainStatusBadge';
import { Button } from '@/components/ui/Button';
import { Card } from '@/components/ui/Card';
import { ConfirmDialog } from '@/components/ui/ConfirmDialog';
import { DataTable, type DataTableColumn } from '@/components/ui/DataTable';
import { LoadingSpinner } from '@/components/ui/LoadingSpinner';
import { PageBackLink } from '@/components/ui/PageBackLink';
import { PageHeader } from '@/components/ui/PageHeader';
import { ApiError } from '@/lib/api/client';
import { fetchCommerce } from '@/lib/api/commerces';
import {
  approveOrder,
  assignDelivery,
  cancelOrder,
  fetchOrder,
  rejectOrder,
  startPicking,
} from '@/lib/api/orders';
import { fetchProductsForPicker } from '@/lib/api/products';
import { fetchDriversForPicker } from '@/lib/api/users';
import type { OrderItem } from '@/lib/api/types';
import { getDeliveryStatusToken, type DeliveryStatus } from '@/lib/admin-tokens';
import { useI18n } from '@/lib/i18n/context';
import { orderActionErrorKey, translateDeliveryStatus } from '@/lib/i18n/labels';
import { formatMoney } from '@/lib/products/formatPrice';
import { buildProductNameMap, productDisplayName } from '@/lib/products/productNameMap';
import { useToast } from '@/hooks/useToast';

export const Route = createFileRoute('/_authenticated/orders/$id')({
  component: OrderDetailPage,
});

function OrderDetailPage() {
  const { id } = Route.useParams();
  const { t } = useI18n();
  const queryClient = useQueryClient();
  const toast = useToast();

  const [rejectOpen, setRejectOpen] = useState(false);
  const [assignOpen, setAssignOpen] = useState(false);
  const [cancelOpen, setCancelOpen] = useState(false);
  const [actionLoading, setActionLoading] = useState(false);

  const order = useQuery({
    queryKey: ['orders', id],
    queryFn: () => fetchOrder(id),
  });

  const commerce = useQuery({
    queryKey: ['commerces', order.data?.commerceId],
    queryFn: () => {
      const commerceId = order.data?.commerceId;
      if (!commerceId) {
        throw new Error('Commerce id missing');
      }
      return fetchCommerce(commerceId);
    },
    enabled: Boolean(order.data?.commerceId),
  });

  const drivers = useQuery({
    queryKey: ['users', 'drivers', 'picker'],
    queryFn: fetchDriversForPicker,
    enabled: assignOpen,
  });

  const products = useQuery({
    queryKey: ['products', 'picker'],
    queryFn: fetchProductsForPicker,
  });

  const productNames = buildProductNameMap(products.data ?? []);

  const lineItemColumns: DataTableColumn<OrderItem>[] = useMemo(
    () => [
      {
        id: 'product',
        header: t('common.table.product'),
        cell: (row) => (
          <Link
            to="/products/$id"
            params={{ id: row.productId }}
            className="text-sm hover:underline"
            onClick={(event) => {
              event.stopPropagation();
            }}
          >
            {productDisplayName(productNames, row.productId)}
          </Link>
        ),
      },
      {
        id: 'quantity',
        header: t('common.table.qty'),
        align: 'right',
        cell: (row) => row.quantity,
      },
      {
        id: 'unitPrice',
        header: t('common.table.unitPrice'),
        align: 'right',
        cell: (row) => formatMoney(row.unitPriceAmount, row.unitPriceCurrency),
      },
      {
        id: 'lineTotal',
        header: t('common.table.lineTotal'),
        align: 'right',
        cell: (row) => formatMoney(row.lineTotalAmount, row.unitPriceCurrency),
      },
    ],
    [t, productNames],
  );

  async function invalidateOrder() {
    await queryClient.invalidateQueries({ queryKey: ['orders'] });
    await queryClient.invalidateQueries({ queryKey: ['orders', id] });
    await queryClient.invalidateQueries({ queryKey: ['deliveries'] });
  }

  async function runAction(action: () => Promise<unknown>, successMessage: string) {
    setActionLoading(true);
    try {
      await action();
      await invalidateOrder();
      toast.success(successMessage);
    } catch (error) {
      const message =
        error instanceof ApiError ? t(orderActionErrorKey(error.code)) : t('errors.actionFailed');
      toast.error(message);
    } finally {
      setActionLoading(false);
    }
  }

  if (order.isLoading) {
    return (
      <div className="flex justify-center py-16">
        <LoadingSpinner />
      </div>
    );
  }

  if (!order.data) {
    return (
      <PageHeader
        title={t('orders.detail.notFound')}
        back={<PageBackLink label={t('common.backTo.orders')} to="/orders" />}
      />
    );
  }

  const detail = order.data;
  const status = detail.status;
  const canApprove = status === 'PendingApproval';
  const canReject = status === 'PendingApproval';
  const canStartPicking = status === 'Approved';
  const canAssignDelivery = (status === 'Approved' || status === 'Picking') && !detail.delivery;
  const canCancel = status === 'Approved' || status === 'Picking';

  return (
    <div className="space-y-6">
      <PageHeader
        title={`${t('forms.fields.order')} ${detail.id.slice(0, 8)}…`}
        description={commerce.data?.tradeName || commerce.data?.legalName || undefined}
        back={<PageBackLink label={t('common.backTo.orders')} to="/orders" />}
        actions={
          <div className="flex flex-wrap gap-2">
            {canApprove ? (
              <Button
                disabled={actionLoading}
                onClick={() => void runAction(() => approveOrder(id), t('orders.toast.approved'))}
              >
                {t('orders.detail.actions.approve')}
              </Button>
            ) : null}
            {canReject ? (
              <Button
                variant="danger"
                disabled={actionLoading}
                onClick={() => {
                  setRejectOpen(true);
                }}
              >
                {t('orders.detail.actions.reject')}
              </Button>
            ) : null}
            {canStartPicking ? (
              <Button
                variant="secondary"
                disabled={actionLoading}
                onClick={() =>
                  void runAction(() => startPicking(id), t('orders.detail.actions.startPicking'))
                }
              >
                {t('orders.detail.actions.startPicking')}
              </Button>
            ) : null}
            {canAssignDelivery ? (
              <Button
                variant="secondary"
                disabled={actionLoading}
                onClick={() => {
                  setAssignOpen(true);
                }}
              >
                {t('orders.detail.actions.assignDelivery')}
              </Button>
            ) : null}
            {canCancel ? (
              <Button
                variant="danger"
                disabled={actionLoading}
                onClick={() => {
                  setCancelOpen(true);
                }}
              >
                {t('orders.detail.actions.cancel')}
              </Button>
            ) : null}
          </div>
        }
      />

      <Card className="space-y-3 p-5">
        <DetailRow
          label={t('forms.fields.status')}
          value={<OrderStatusBadge status={detail.status} />}
        />
        <DetailRow
          label={t('forms.fields.commerce')}
          value={
            commerce.data ? (
              <Link
                to="/commerces/$id"
                params={{ id: detail.commerceId }}
                className="hover:underline"
              >
                {commerce.data.tradeName || commerce.data.legalName}
              </Link>
            ) : (
              detail.commerceId
            )
          }
        />
        <DetailRow
          label={t('common.table.total')}
          value={formatMoney(detail.totalAmount, detail.totalCurrency)}
        />
        {detail.notes ? <DetailRow label={t('forms.fields.notes')} value={detail.notes} /> : null}
        {detail.rejectionReason ? (
          <DetailRow label={t('forms.fields.rejectionReason')} value={detail.rejectionReason} />
        ) : null}
      </Card>

      {detail.delivery ? (
        <Card className="space-y-3 p-5">
          <h2 className="text-base font-semibold text-foreground">
            {t('orders.detail.deliverySection')}
          </h2>
          <DetailRow
            label={t('forms.fields.delivery')}
            value={
              <Link
                to="/deliveries/$id"
                params={{ id: detail.delivery.id }}
                className="font-mono text-xs hover:underline"
              >
                {detail.delivery.id.slice(0, 8)}…
              </Link>
            }
          />
          <DetailRow
            label={t('forms.fields.status')}
            value={
              <DomainStatusBadge
                colors={getDeliveryStatusToken(detail.delivery.status as DeliveryStatus)}
                label={translateDeliveryStatus(t, detail.delivery.status as DeliveryStatus)}
              />
            }
          />
        </Card>
      ) : null}

      <div>
        <h2 className="mb-3 text-base font-semibold text-foreground">
          {t('forms.sections.lineItems')}
        </h2>
        <DataTable
          caption={t('orders.detail.lineItems')}
          columns={lineItemColumns}
          rows={detail.items}
          getRowKey={(row) => row.id}
          density="compact"
        />
      </div>

      <RejectOrderDialog
        open={rejectOpen}
        isLoading={actionLoading}
        onCancel={() => {
          setRejectOpen(false);
        }}
        onConfirm={(reason) => {
          void (async () => {
            setActionLoading(true);
            try {
              await rejectOrder(id, reason);
              await invalidateOrder();
              toast.success(t('orders.toast.rejected'));
              setRejectOpen(false);
            } catch (error) {
              const message =
                error instanceof ApiError
                  ? t(orderActionErrorKey(error.code))
                  : t('errors.actionFailed');
              toast.error(message);
            } finally {
              setActionLoading(false);
            }
          })();
        }}
      />

      <AssignDeliveryDialog
        open={assignOpen}
        drivers={drivers.data ?? []}
        isLoading={actionLoading}
        onCancel={() => {
          setAssignOpen(false);
        }}
        onConfirm={(driverId) => {
          void (async () => {
            setActionLoading(true);
            try {
              await assignDelivery(id, driverId);
              await invalidateOrder();
              toast.success(t('orders.toast.deliveryAssigned'));
              setAssignOpen(false);
            } catch (error) {
              const message =
                error instanceof ApiError
                  ? t(orderActionErrorKey(error.code))
                  : t('errors.actionFailed');
              toast.error(message);
            } finally {
              setActionLoading(false);
            }
          })();
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
          setCancelOpen(false);
        }}
        onConfirm={() => {
          void (async () => {
            await runAction(() => cancelOrder(id), t('orders.toast.cancelled'));
            setCancelOpen(false);
          })();
        }}
      />
    </div>
  );
}

function DetailRow({ label, value }: { label: string; value: ReactNode }) {
  return (
    <div className="flex flex-col gap-1 sm:flex-row sm:items-center sm:justify-between">
      <span className="text-xs font-semibold uppercase tracking-[0.12em] text-muted-foreground">
        {label}
      </span>
      <span className="text-sm text-foreground">{value}</span>
    </div>
  );
}

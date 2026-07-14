import { createFileRoute } from '@tanstack/react-router';
import { useQuery, useQueryClient } from '@tanstack/react-query';
import { useMemo, useState } from 'react';

import { useProductLineColumns } from '@/components/detail/productLineColumns';
import { OrderDetailActions } from '@/components/orders/OrderDetailActions';
import { OrderDetailSummary } from '@/components/orders/OrderDetailSummary';
import { OrderWorkflowDialogs } from '@/components/orders/OrderWorkflowDialogs';
import { DataTable } from '@/components/ui/DataTable';
import { DetailSectionCard } from '@/components/ui/DetailFields';
import { LoadingSpinner } from '@/components/ui/LoadingSpinner';
import { PageBackLink } from '@/components/ui/PageBackLink';
import { PageHeader } from '@/components/ui/PageHeader';
import { useToast } from '@/hooks/useToast';
import { ApiError } from '@/lib/api/client';
import { fetchCommerce } from '@/lib/api/commerces';
import { approveOrder, fetchOrder, startPicking } from '@/lib/api/orders';
import { fetchProductsForPicker } from '@/lib/api/products';
import { fetchDriversForPicker } from '@/lib/api/users';
import { useI18n } from '@/lib/i18n/context';
import { orderActionErrorKey } from '@/lib/i18n/labels';
import { buildProductNameMap } from '@/lib/products/productNameMap';

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

  const order = useQuery({ queryKey: ['orders', id], queryFn: () => fetchOrder(id) });
  const commerce = useQuery({
    queryKey: ['commerces', order.data?.commerceId],
    queryFn: () => {
      const commerceId = order.data?.commerceId;
      if (!commerceId) {
        return Promise.reject(new Error('commerceId required'));
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
  const products = useQuery({ queryKey: ['products', 'picker'], queryFn: fetchProductsForPicker });
  const productNames = buildProductNameMap(products.data ?? []);
  const lineLabels = useMemo(
    () => ({
      product: t('common.table.product'),
      qty: t('common.table.qty'),
      unitPrice: t('common.table.unitPrice'),
      lineTotal: t('common.table.lineTotal'),
    }),
    [t],
  );
  const lineItemColumns = useProductLineColumns(lineLabels, productNames);

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
  const commerceName = commerce.data?.tradeName || commerce.data?.legalName;

  return (
    <div className="space-y-6">
      <PageHeader
        title={`${t('forms.fields.order')} ${detail.id.slice(0, 8)}…`}
        description={commerceName || undefined}
        back={<PageBackLink label={t('common.backTo.orders')} to="/orders" />}
        actions={
          <OrderDetailActions
            status={detail.status}
            hasDelivery={Boolean(detail.delivery)}
            actionLoading={actionLoading}
            onApprove={() => void runAction(() => approveOrder(id), t('orders.toast.approved'))}
            onReject={() => {
              setRejectOpen(true);
            }}
            onStartPicking={() =>
              void runAction(() => startPicking(id), t('orders.detail.actions.startPicking'))
            }
            onAssignDelivery={() => {
              setAssignOpen(true);
            }}
            onCancel={() => {
              setCancelOpen(true);
            }}
          />
        }
      />

      <OrderDetailSummary
        detail={detail}
        commerceName={commerceName}
        commerceReady={Boolean(commerce.data)}
      />

      <DetailSectionCard title={t('forms.sections.lineItems')}>
        <DataTable
          caption={t('orders.detail.lineItems')}
          columns={lineItemColumns}
          rows={detail.items}
          getRowKey={(row) => row.id}
          density="compact"
          className="rounded-none border-0"
        />
      </DetailSectionCard>

      <OrderWorkflowDialogs
        orderId={id}
        rejectOpen={rejectOpen}
        assignOpen={assignOpen}
        cancelOpen={cancelOpen}
        drivers={drivers.data ?? []}
        actionLoading={actionLoading}
        onRejectOpenChange={setRejectOpen}
        onAssignOpenChange={setAssignOpen}
        onCancelOpenChange={setCancelOpen}
        onActionLoadingChange={setActionLoading}
        onInvalidate={invalidateOrder}
      />
    </div>
  );
}

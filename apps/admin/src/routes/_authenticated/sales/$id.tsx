import { createFileRoute } from '@tanstack/react-router';
import { useQuery, useQueryClient } from '@tanstack/react-query';
import { useMemo, useState } from 'react';

import { useProductLineColumns } from '@/components/detail/productLineColumns';
import { SaleDetailSummary } from '@/components/sales/SaleDetailSummary';
import { Button } from '@/components/ui/Button';
import { ConfirmDialog } from '@/components/ui/ConfirmDialog';
import { DataTable } from '@/components/ui/DataTable';
import { DetailSectionCard } from '@/components/ui/DetailFields';
import { LoadingSpinner } from '@/components/ui/LoadingSpinner';
import { PageBackLink } from '@/components/ui/PageBackLink';
import { PageHeader } from '@/components/ui/PageHeader';
import { useToast } from '@/hooks/useToast';
import { ApiError } from '@/lib/api/client';
import { fetchCommerce } from '@/lib/api/commerces';
import { fetchProductsForPicker } from '@/lib/api/products';
import { cancelSale, confirmSale, fetchSale } from '@/lib/api/sales';
import { fetchUser } from '@/lib/api/users';
import { useI18n } from '@/lib/i18n/context';
import { saleActionErrorKey } from '@/lib/i18n/labels';
import { buildProductNameMap } from '@/lib/products/productNameMap';

export const Route = createFileRoute('/_authenticated/sales/$id')({
  component: SaleDetailPage,
});

function SaleDetailPage() {
  const { id } = Route.useParams();
  const { t } = useI18n();
  const queryClient = useQueryClient();
  const toast = useToast();
  const [cancelOpen, setCancelOpen] = useState(false);
  const [actionLoading, setActionLoading] = useState(false);

  const sale = useQuery({ queryKey: ['sales', id], queryFn: () => fetchSale(id) });
  const commerce = useQuery({
    queryKey: ['commerces', sale.data?.commerceId],
    queryFn: () => {
      const commerceId = sale.data?.commerceId;
      if (!commerceId) {
        return Promise.reject(new Error('commerceId required'));
      }
      return fetchCommerce(commerceId);
    },
    enabled: Boolean(sale.data?.commerceId),
  });
  const driver = useQuery({
    queryKey: ['users', sale.data?.driverId],
    queryFn: () => {
      const driverId = sale.data?.driverId;
      if (!driverId) {
        return Promise.reject(new Error('driverId required'));
      }
      return fetchUser(driverId);
    },
    enabled: Boolean(sale.data?.driverId),
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

  async function invalidateSale() {
    await queryClient.invalidateQueries({ queryKey: ['sales'] });
    await queryClient.invalidateQueries({ queryKey: ['sales', id] });
  }

  async function runAction(action: () => Promise<unknown>, successMessage: string) {
    setActionLoading(true);
    try {
      await action();
      await invalidateSale();
      toast.success(successMessage);
    } catch (error) {
      const message =
        error instanceof ApiError ? t(saleActionErrorKey(error.code)) : t('errors.actionFailed');
      toast.error(message);
    } finally {
      setActionLoading(false);
    }
  }

  if (sale.isLoading) {
    return (
      <div className="flex justify-center py-16">
        <LoadingSpinner />
      </div>
    );
  }

  if (!sale.data) {
    return (
      <PageHeader
        title={t('sales.detail.notFound')}
        back={<PageBackLink label={t('common.backTo.sales')} to="/sales" />}
      />
    );
  }

  const detail = sale.data;
  const commerceName = commerce.data?.tradeName || commerce.data?.legalName;
  const isPending = detail.status === 'Pending';

  return (
    <div className="space-y-6">
      <PageHeader
        title={`${t('forms.fields.sale')} ${detail.id.slice(0, 8)}…`}
        description={commerceName || undefined}
        back={<PageBackLink label={t('common.backTo.sales')} to="/sales" />}
        actions={
          isPending ? (
            <div className="flex flex-wrap gap-2">
              <Button
                variant="success"
                disabled={actionLoading}
                onClick={() => void runAction(() => confirmSale(id), t('sales.toast.confirmed'))}
              >
                {t('sales.detail.actions.confirm')}
              </Button>
              <Button
                variant="danger"
                disabled={actionLoading}
                onClick={() => {
                  setCancelOpen(true);
                }}
              >
                {t('sales.detail.actions.cancel')}
              </Button>
            </div>
          ) : null
        }
      />

      <SaleDetailSummary
        detail={detail}
        commerceName={commerceName}
        commerceReady={Boolean(commerce.data)}
        driverName={driver.data?.name}
        driverReady={Boolean(driver.data)}
      />

      <DetailSectionCard title={t('forms.sections.lineItems')}>
        <DataTable
          caption={t('sales.detail.lineItems')}
          columns={lineItemColumns}
          rows={detail.items}
          getRowKey={(row) => row.productId}
          density="compact"
          className="rounded-none border-0"
        />
      </DetailSectionCard>

      <ConfirmDialog
        open={cancelOpen}
        title={t('sales.cancelDialog.title')}
        message={t('sales.cancelDialog.message')}
        confirmLabel={t('sales.cancelDialog.confirm')}
        destructive
        isLoading={actionLoading}
        onCancel={() => {
          setCancelOpen(false);
        }}
        onConfirm={() => {
          void (async () => {
            await runAction(() => cancelSale(id), t('sales.toast.cancelled'));
            setCancelOpen(false);
          })();
        }}
      />
    </div>
  );
}

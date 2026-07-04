import { Link, createFileRoute } from '@tanstack/react-router';
import { useQuery, useQueryClient } from '@tanstack/react-query';
import { useMemo, useState, type ReactNode } from 'react';

import { DeclaredPaymentBadge } from '@/components/sales/DeclaredPaymentBadge';
import { PaymentMethodBadge } from '@/components/sales/PaymentMethodBadge';
import { SaleStatusBadge } from '@/components/sales/SaleStatusBadge';
import { Button } from '@/components/ui/Button';
import { Card } from '@/components/ui/Card';
import { ConfirmDialog } from '@/components/ui/ConfirmDialog';
import { DataTable, type DataTableColumn } from '@/components/ui/DataTable';
import { LoadingSpinner } from '@/components/ui/LoadingSpinner';
import { PageBackLink } from '@/components/ui/PageBackLink';
import { PageHeader } from '@/components/ui/PageHeader';
import { useToast } from '@/hooks/useToast';
import { ApiError } from '@/lib/api/client';
import { fetchCommerce } from '@/lib/api/commerces';
import { fetchProductsForPicker } from '@/lib/api/products';
import { cancelSale, confirmSale, fetchSale } from '@/lib/api/sales';
import { fetchUser } from '@/lib/api/users';
import type { SaleItem } from '@/lib/api/types';
import { useI18n } from '@/lib/i18n/context';
import { saleActionErrorKey } from '@/lib/i18n/labels';
import { formatMoney } from '@/lib/products/formatPrice';
import { buildProductNameMap, productDisplayName } from '@/lib/products/productNameMap';

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

  const sale = useQuery({
    queryKey: ['sales', id],
    queryFn: () => fetchSale(id),
  });

  const commerce = useQuery({
    queryKey: ['commerces', sale.data?.commerceId],
    queryFn: () => {
      const commerceId = sale.data?.commerceId;
      if (!commerceId) {
        throw new Error('Commerce id missing');
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
        throw new Error('Driver id missing');
      }
      return fetchUser(driverId);
    },
    enabled: Boolean(sale.data?.driverId),
  });

  const products = useQuery({
    queryKey: ['products', 'picker'],
    queryFn: fetchProductsForPicker,
  });

  const productNames = buildProductNameMap(products.data ?? []);

  const lineItemColumns: DataTableColumn<SaleItem>[] = useMemo(
    () => [
      {
        id: 'product',
        header: t('common.table.product'),
        cell: (row) => (
          <Link
            to="/products/$id"
            params={{ id: row.productId }}
            className="text-sm hover:underline"
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
  const isPending = detail.status === 'Pending';

  return (
    <div className="space-y-6">
      <PageHeader
        title={`${t('forms.fields.sale')} ${detail.id.slice(0, 8)}…`}
        description={commerce.data?.tradeName || commerce.data?.legalName || undefined}
        back={<PageBackLink label={t('common.backTo.sales')} to="/sales" />}
        actions={
          isPending ? (
            <div className="flex flex-wrap gap-2">
              <Button
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

      <Card className="space-y-3 p-5">
        <DetailRow
          label={t('forms.fields.status')}
          value={<SaleStatusBadge status={detail.status} />}
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
          label={t('forms.fields.driver')}
          value={
            driver.data ? (
              <Link to="/users/$id" params={{ id: detail.driverId }} className="hover:underline">
                {driver.data.name}
              </Link>
            ) : (
              detail.driverId
            )
          }
        />
        {detail.orderId ? (
          <DetailRow
            label={t('forms.fields.order')}
            value={
              <Link
                to="/orders/$id"
                params={{ id: detail.orderId }}
                className="font-mono text-xs hover:underline"
              >
                {detail.orderId.slice(0, 8)}…
              </Link>
            }
          />
        ) : null}
        <DetailRow
          label={t('forms.fields.paymentMethod')}
          value={<PaymentMethodBadge method={detail.paymentMethod} />}
        />
        <DetailRow
          label={t('forms.fields.declaredPayment')}
          value={
            <DeclaredPaymentBadge
              method={detail.declaredPaymentMethod}
              received={detail.declaredPaymentReceived}
            />
          }
        />
        <DetailRow
          label={t('common.table.total')}
          value={formatMoney(detail.totalAmount, detail.totalCurrency)}
        />
      </Card>

      <div>
        <h2 className="mb-3 text-base font-semibold text-foreground">
          {t('forms.sections.lineItems')}
        </h2>
        <DataTable
          caption={t('sales.detail.lineItems')}
          columns={lineItemColumns}
          rows={detail.items}
          getRowKey={(row) => row.productId}
          density="compact"
        />
      </div>

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

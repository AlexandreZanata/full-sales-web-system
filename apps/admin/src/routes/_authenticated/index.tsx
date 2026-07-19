import { createFileRoute, Link } from '@tanstack/react-router';
import { useQuery } from '@tanstack/react-query';
import { ArrowRight, ShoppingCart, Truck } from 'lucide-react';

import { SaleStatusBadge } from '@/components/sales/SaleStatusBadge';
import { Button } from '@/components/ui/Button';
import { Card } from '@/components/ui/Card';
import { DataTable, type DataTableColumn } from '@/components/ui/DataTable';
import { EmptyState } from '@/components/ui/EmptyState';
import { LoadingSpinner } from '@/components/ui/LoadingSpinner';
import { PageHeader } from '@/components/ui/PageHeader';
import {
  fetchPendingOrdersCount,
  fetchRecentSales,
  fetchWaitingDeliveriesCount,
} from '@/lib/api/dashboard';
import type { SaleSummary } from '@/lib/api/types';
import { useI18n } from '@/lib/i18n/context';
import { saleDisplayCode } from '@/lib/sales/saleDisplayCode';

export const Route = createFileRoute('/_authenticated/')({
  component: DashboardPage,
});

function formatMoney(amount: number, currency: string): string {
  return new Intl.NumberFormat(undefined, {
    style: 'currency',
    currency: currency || 'BRL',
  }).format(amount / 100);
}

function formatDateTime(value: string): string {
  return new Intl.DateTimeFormat(undefined, {
    dateStyle: 'medium',
    timeStyle: 'short',
  }).format(new Date(value));
}

function StatCard({
  title,
  value,
  loading,
  to,
  icon: Icon,
  viewAllLabel,
}: {
  title: string;
  value: number | null;
  loading: boolean;
  to: '/orders' | '/deliveries' | '/sales';
  icon: typeof ShoppingCart;
  viewAllLabel: string;
}) {
  return (
    <Card className="flex flex-col gap-3 p-5">
      <div className="flex items-start justify-between gap-3">
        <div>
          <p className="text-xs font-semibold uppercase tracking-[0.12em] text-muted-foreground">
            {title}
          </p>
          <p className="mt-2 text-3xl font-semibold tabular-nums text-foreground">
            {loading ? '—' : (value ?? 0)}
          </p>
        </div>
        <span className="rounded-md border border-hairline bg-surface-muted p-2 text-muted-foreground">
          <Icon className="size-5" aria-hidden />
        </span>
      </div>
      <Link
        to={to}
        className="inline-flex items-center gap-1 text-sm font-medium text-foreground hover:underline"
      >
        {viewAllLabel}
        <ArrowRight className="size-4" aria-hidden />
      </Link>
    </Card>
  );
}

function DashboardPage() {
  const { t } = useI18n();
  const pendingOrders = useQuery({
    queryKey: ['dashboard', 'pendingOrders'],
    queryFn: fetchPendingOrdersCount,
  });
  const waitingDeliveries = useQuery({
    queryKey: ['dashboard', 'waitingDeliveries'],
    queryFn: fetchWaitingDeliveriesCount,
  });
  const recentSales = useQuery({
    queryKey: ['dashboard', 'recentSales'],
    queryFn: () => fetchRecentSales(5),
  });

  const saleColumns: DataTableColumn<SaleSummary>[] = [
    {
      id: 'code',
      header: t('common.table.code'),
      cell: (row) => <span className="font-mono text-xs">{saleDisplayCode(row)}</span>,
    },
    {
      id: 'createdAt',
      header: t('common.table.date'),
      cell: (row) => formatDateTime(row.createdAt),
    },
    {
      id: 'status',
      header: t('common.table.status'),
      cell: (row) => <SaleStatusBadge status={row.status} />,
    },
    {
      id: 'total',
      header: t('common.table.total'),
      align: 'right',
      cell: (row) => formatMoney(row.totalAmount, row.totalCurrency),
    },
  ];

  return (
    <div className="space-y-6">
      <PageHeader title={t('dashboard.title')} description={t('dashboard.description')} />

      <div className="grid gap-4 sm:grid-cols-2">
        <StatCard
          title={t('dashboard.stats.pendingApproval')}
          value={pendingOrders.data ?? null}
          loading={pendingOrders.isLoading}
          to="/orders"
          icon={ShoppingCart}
          viewAllLabel={t('common.viewAll')}
        />
        <StatCard
          title={t('dashboard.stats.deliveriesWaiting')}
          value={waitingDeliveries.data ?? null}
          loading={waitingDeliveries.isLoading}
          to="/deliveries"
          icon={Truck}
          viewAllLabel={t('common.viewAll')}
        />
      </div>

      <Card className="p-0">
        <div className="border-b border-hairline px-5 py-4">
          <div className="flex items-center justify-between gap-3">
            <h2 className="text-base font-semibold text-foreground">
              {t('dashboard.recentSales.title')}
            </h2>
            <Link to="/sales">
              <Button variant="secondary" type="button">
                {t('dashboard.recentSales.viewAll')}
              </Button>
            </Link>
          </div>
        </div>

        {recentSales.isLoading ? (
          <div className="flex justify-center py-10">
            <LoadingSpinner />
          </div>
        ) : recentSales.data && recentSales.data.length > 0 ? (
          <DataTable
            caption={t('dashboard.recentSales.caption')}
            columns={saleColumns}
            rows={recentSales.data}
            getRowKey={(row) => row.id}
            density="compact"
            className="border-0"
          />
        ) : (
          <EmptyState
            title={t('dashboard.recentSales.empty.title')}
            description={t('dashboard.recentSales.empty.description')}
          />
        )}
      </Card>
    </div>
  );
}

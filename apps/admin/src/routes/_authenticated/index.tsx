import { createFileRoute, Link } from '@tanstack/react-router';
import { useQuery } from '@tanstack/react-query';
import { ArrowRight, ShoppingCart, Truck } from 'lucide-react';

import { DomainStatusBadge } from '@/components/status/DomainStatusBadge';
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
import { getSaleStatusToken, type SaleStatus } from '@/lib/admin-tokens';

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

const saleColumns: DataTableColumn<SaleSummary>[] = [
  {
    id: 'createdAt',
    header: 'Date',
    cell: (row) => formatDateTime(row.createdAt),
  },
  {
    id: 'status',
    header: 'Status',
    cell: (row) => <DomainStatusBadge colors={getSaleStatusToken(row.status as SaleStatus)} />,
  },
  {
    id: 'total',
    header: 'Total',
    align: 'right',
    cell: (row) => formatMoney(row.totalAmount, row.totalCurrency),
  },
];

function StatCard({
  title,
  value,
  loading,
  to,
  icon: Icon,
}: {
  title: string;
  value: number | null;
  loading: boolean;
  to: '/orders' | '/deliveries' | '/sales';
  icon: typeof ShoppingCart;
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
        View all
        <ArrowRight className="size-4" aria-hidden />
      </Link>
    </Card>
  );
}

function DashboardPage() {
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

  return (
    <div className="space-y-6">
      <PageHeader
        title="Dashboard"
        description="Overview of pending approvals, deliveries, and recent sales."
      />

      <div className="grid gap-4 sm:grid-cols-2">
        <StatCard
          title="Pending approval"
          value={pendingOrders.data ?? null}
          loading={pendingOrders.isLoading}
          to="/orders"
          icon={ShoppingCart}
        />
        <StatCard
          title="Deliveries waiting"
          value={waitingDeliveries.data ?? null}
          loading={waitingDeliveries.isLoading}
          to="/deliveries"
          icon={Truck}
        />
      </div>

      <Card className="p-0">
        <div className="border-b border-hairline px-5 py-4">
          <div className="flex items-center justify-between gap-3">
            <h2 className="text-base font-semibold text-foreground">Recent sales</h2>
            <Link to="/sales">
              <Button variant="secondary" type="button">
                All sales
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
            caption="Recent sales"
            columns={saleColumns}
            rows={recentSales.data}
            getRowKey={(row) => row.id}
            density="compact"
            className="border-0"
          />
        ) : (
          <EmptyState title="No recent sales" description="Sales will appear here once recorded." />
        )}
      </Card>
    </div>
  );
}

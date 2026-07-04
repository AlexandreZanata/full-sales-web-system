import { createFileRoute, useNavigate } from '@tanstack/react-router';
import { useQuery } from '@tanstack/react-query';
import { useMemo, useState } from 'react';

import { OrderStatusBadge } from '@/components/orders/OrderStatusBadge';
import { DataTable, type DataTableColumn } from '@/components/ui/DataTable';
import { EmptyState } from '@/components/ui/EmptyState';
import { Input } from '@/components/ui/Input';
import { LoadingSpinner } from '@/components/ui/LoadingSpinner';
import { PageHeader } from '@/components/ui/PageHeader';
import { Select } from '@/components/ui/Select';
import { fetchCommercesForPicker } from '@/lib/api/commerces';
import { dateFilterToIso, fetchOrders } from '@/lib/api/orders';
import type { OrderSummary } from '@/lib/api/types';
import { formatDateTime } from '@/lib/formatDateTime';
import { ORDER_STATUS_FILTER_LABELS, type OrderStatusFilter } from '@/lib/orders/constants';
import { formatMoney } from '@/lib/products/formatPrice';
import { paginatedResponseToTable } from '@/lib/tablePagination';

export const Route = createFileRoute('/_authenticated/orders/')({
  component: OrdersListPage,
});

function OrdersListPage() {
  const navigate = useNavigate();
  const [page, setPage] = useState(1);
  const [statusFilter, setStatusFilter] = useState<OrderStatusFilter>('');
  const [commerceFilter, setCommerceFilter] = useState('');
  const [fromDate, setFromDate] = useState('');
  const [toDate, setToDate] = useState('');
  const pageSize = 20;

  const commerces = useQuery({
    queryKey: ['commerces', 'picker'],
    queryFn: fetchCommercesForPicker,
  });

  const commerceNames = useMemo(() => {
    const map = new Map<string, string>();
    for (const commerce of commerces.data ?? []) {
      map.set(commerce.id, commerce.tradeName || commerce.legalName);
    }
    return map;
  }, [commerces.data]);

  const orders = useQuery({
    queryKey: ['orders', page, pageSize, statusFilter, commerceFilter, fromDate, toDate],
    queryFn: () =>
      fetchOrders({
        page,
        pageSize,
        status: statusFilter,
        commerceId: commerceFilter || undefined,
        from: fromDate ? dateFilterToIso(fromDate, 'start') : undefined,
        to: toDate ? dateFilterToIso(toDate, 'end') : undefined,
      }),
  });

  const pagination = orders.data ? paginatedResponseToTable(orders.data) : null;

  const columns: DataTableColumn<OrderSummary>[] = [
    {
      id: 'id',
      header: 'Order',
      cell: (row) => <span className="font-mono text-xs">{row.id.slice(0, 8)}…</span>,
    },
    {
      id: 'status',
      header: 'Status',
      cell: (row) => <OrderStatusBadge status={row.status} />,
    },
    {
      id: 'commerce',
      header: 'Commerce',
      cell: (row) => commerceNames.get(row.commerceId) ?? row.commerceId.slice(0, 8),
    },
    {
      id: 'total',
      header: 'Total',
      align: 'right',
      cell: (row) => formatMoney(row.totalAmount, row.totalCurrency),
    },
    {
      id: 'createdAt',
      header: 'Created',
      cell: (row) => formatDateTime(row.createdAt),
    },
  ];

  return (
    <div>
      <PageHeader
        title="Orders"
        description="Review and manage the order lifecycle from approval through picking."
      />

      <div className="mb-4 grid gap-4 sm:grid-cols-2 lg:grid-cols-4">
        <Select
          label="Status"
          value={statusFilter}
          onChange={(event) => {
            setStatusFilter(event.target.value as OrderStatusFilter);
            setPage(1);
          }}
        >
          {(Object.keys(ORDER_STATUS_FILTER_LABELS) as OrderStatusFilter[]).map((value) => (
            <option key={value || 'all'} value={value}>
              {ORDER_STATUS_FILTER_LABELS[value]}
            </option>
          ))}
        </Select>

        <Select
          label="Commerce"
          value={commerceFilter}
          onChange={(event) => {
            setCommerceFilter(event.target.value);
            setPage(1);
          }}
        >
          <option value="">All commerces</option>
          {(commerces.data ?? []).map((commerce) => (
            <option key={commerce.id} value={commerce.id}>
              {commerce.tradeName || commerce.legalName}
            </option>
          ))}
        </Select>

        <Input
          label="From"
          type="date"
          value={fromDate}
          onChange={(event) => {
            setFromDate(event.target.value);
            setPage(1);
          }}
        />

        <Input
          label="To"
          type="date"
          value={toDate}
          onChange={(event) => {
            setToDate(event.target.value);
            setPage(1);
          }}
        />
      </div>

      {orders.isLoading ? (
        <div className="flex justify-center py-16">
          <LoadingSpinner />
        </div>
      ) : orders.data && orders.data.items.length > 0 ? (
        <DataTable
          caption="Orders"
          columns={columns}
          rows={orders.data.items}
          getRowKey={(row) => row.id}
          pagination={pagination}
          onPageChange={setPage}
          onRowClick={(row) => {
            void navigate({ to: '/orders/$id', params: { id: row.id } });
          }}
        />
      ) : (
        <EmptyState
          title="No orders found"
          description={
            statusFilter || commerceFilter || fromDate || toDate
              ? 'Try adjusting filters or check back when new orders are submitted.'
              : 'Orders will appear here once submitted from the commerce portal.'
          }
        />
      )}
    </div>
  );
}

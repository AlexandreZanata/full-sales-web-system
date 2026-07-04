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
import { useI18n } from '@/lib/i18n/context';
import { orderStatusFilterLabel } from '@/lib/i18n/labels';
import { ORDER_STATUS_FILTERS, type OrderStatusFilter } from '@/lib/orders/constants';
import { formatMoney } from '@/lib/products/formatPrice';
import { paginatedResponseToTable } from '@/lib/tablePagination';

export const Route = createFileRoute('/_authenticated/orders/')({
  component: OrdersListPage,
});

function OrdersListPage() {
  const { t } = useI18n();
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

  const columns: DataTableColumn<OrderSummary>[] = useMemo(
    () => [
      {
        id: 'id',
        header: t('forms.fields.order'),
        cell: (row) => <span className="font-mono text-xs">{row.id.slice(0, 8)}…</span>,
      },
      {
        id: 'status',
        header: t('common.table.status'),
        cell: (row) => <OrderStatusBadge status={row.status} />,
      },
      {
        id: 'commerce',
        header: t('forms.fields.commerce'),
        cell: (row) => commerceNames.get(row.commerceId) ?? row.commerceId.slice(0, 8),
      },
      {
        id: 'total',
        header: t('common.table.total'),
        align: 'right',
        cell: (row) => formatMoney(row.totalAmount, row.totalCurrency),
      },
      {
        id: 'createdAt',
        header: t('common.table.date'),
        cell: (row) => formatDateTime(row.createdAt),
      },
    ],
    [t, commerceNames],
  );

  return (
    <div>
      <PageHeader title={t('orders.list.title')} description={t('orders.list.description')} />

      <div className="mb-4 grid gap-4 sm:grid-cols-2 lg:grid-cols-4">
        <Select
          label={t('forms.fields.status')}
          value={statusFilter}
          onChange={(event) => {
            setStatusFilter(event.target.value as OrderStatusFilter);
            setPage(1);
          }}
        >
          {ORDER_STATUS_FILTERS.map((value) => (
            <option key={value || 'all'} value={value}>
              {orderStatusFilterLabel(t, value)}
            </option>
          ))}
        </Select>

        <Select
          label={t('forms.fields.commerce')}
          value={commerceFilter}
          onChange={(event) => {
            setCommerceFilter(event.target.value);
            setPage(1);
          }}
        >
          <option value="">{t('common.filter.allCommerces')}</option>
          {(commerces.data ?? []).map((commerce) => (
            <option key={commerce.id} value={commerce.id}>
              {commerce.tradeName || commerce.legalName}
            </option>
          ))}
        </Select>

        <Input
          label={t('forms.fields.from')}
          type="date"
          value={fromDate}
          onChange={(event) => {
            setFromDate(event.target.value);
            setPage(1);
          }}
        />

        <Input
          label={t('forms.fields.to')}
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
          caption={t('orders.list.caption')}
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
          title={t('orders.list.empty.title')}
          description={
            statusFilter || commerceFilter || fromDate || toDate
              ? t('orders.list.empty.descriptionFiltered')
              : t('orders.list.empty.descriptionDefault')
          }
        />
      )}
    </div>
  );
}

import { Link, createFileRoute, useNavigate } from '@tanstack/react-router';
import { useQuery } from '@tanstack/react-query';
import { useMemo, useState } from 'react';

import { DeclaredPaymentBadge } from '@/components/sales/DeclaredPaymentBadge';
import { PaymentMethodBadge } from '@/components/sales/PaymentMethodBadge';
import { SaleStatusBadge } from '@/components/sales/SaleStatusBadge';
import { DataTable, type DataTableColumn } from '@/components/ui/DataTable';
import { EmptyState } from '@/components/ui/EmptyState';
import { Input } from '@/components/ui/Input';
import { LoadingSpinner } from '@/components/ui/LoadingSpinner';
import { PageHeader } from '@/components/ui/PageHeader';
import { Select } from '@/components/ui/Select';
import { Button } from '@/components/ui/Button';
import { fetchCommercesForPicker } from '@/lib/api/commerces';
import { dateFilterToIso, fetchSales } from '@/lib/api/sales';
import { fetchDriversForPicker } from '@/lib/api/users';
import type { SaleSummary } from '@/lib/api/types';
import { formatDateTime } from '@/lib/formatDateTime';
import { formatMoney } from '@/lib/products/formatPrice';
import { SALE_STATUS_FILTER_LABELS, type SaleStatusFilter } from '@/lib/sales/constants';
import { paginatedResponseToTable } from '@/lib/tablePagination';

export const Route = createFileRoute('/_authenticated/sales/')({
  component: SalesListPage,
});

function SalesListPage() {
  const navigate = useNavigate();
  const [page, setPage] = useState(1);
  const [statusFilter, setStatusFilter] = useState<SaleStatusFilter>('');
  const [commerceFilter, setCommerceFilter] = useState('');
  const [driverFilter, setDriverFilter] = useState('');
  const [fromDate, setFromDate] = useState('');
  const [toDate, setToDate] = useState('');
  const pageSize = 20;

  const commerces = useQuery({
    queryKey: ['commerces', 'picker'],
    queryFn: fetchCommercesForPicker,
  });

  const drivers = useQuery({
    queryKey: ['users', 'drivers', 'picker'],
    queryFn: fetchDriversForPicker,
  });

  const commerceNames = useMemo(() => {
    const map = new Map<string, string>();
    for (const commerce of commerces.data ?? []) {
      map.set(commerce.id, commerce.tradeName || commerce.legalName);
    }
    return map;
  }, [commerces.data]);

  const driverNames = useMemo(() => {
    const map = new Map<string, string>();
    for (const driver of drivers.data ?? []) {
      map.set(driver.id, driver.name);
    }
    return map;
  }, [drivers.data]);

  const sales = useQuery({
    queryKey: [
      'sales',
      page,
      pageSize,
      statusFilter,
      commerceFilter,
      driverFilter,
      fromDate,
      toDate,
    ],
    queryFn: () =>
      fetchSales({
        page,
        pageSize,
        status: statusFilter,
        commerceId: commerceFilter || undefined,
        driverId: driverFilter || undefined,
        from: fromDate ? dateFilterToIso(fromDate, 'start') : undefined,
        to: toDate ? dateFilterToIso(toDate, 'end') : undefined,
      }),
  });

  const pagination = sales.data ? paginatedResponseToTable(sales.data) : null;

  const columns: DataTableColumn<SaleSummary>[] = [
    {
      id: 'createdAt',
      header: 'Date',
      cell: (row) => formatDateTime(row.createdAt),
    },
    {
      id: 'status',
      header: 'Status',
      cell: (row) => <SaleStatusBadge status={row.status} />,
    },
    {
      id: 'commerce',
      header: 'Commerce',
      cell: (row) => commerceNames.get(row.commerceId) ?? row.commerceId.slice(0, 8),
    },
    {
      id: 'driver',
      header: 'Driver',
      cell: (row) => driverNames.get(row.driverId) ?? row.driverId.slice(0, 8),
    },
    {
      id: 'payment',
      header: 'Payment',
      cell: (row) => (
        <div className="flex flex-wrap gap-1">
          <PaymentMethodBadge method={row.paymentMethod} />
          <DeclaredPaymentBadge
            method={row.declaredPaymentMethod}
            received={row.declaredPaymentReceived}
          />
        </div>
      ),
    },
    {
      id: 'total',
      header: 'Total',
      align: 'right',
      cell: (row) => formatMoney(row.totalAmount, row.totalCurrency),
    },
  ];

  return (
    <div>
      <PageHeader
        title="Sales"
        description="Review field sales across the tenant. Admin sees all sales."
        actions={
          <Link to="/sales/new">
            <Button>New sale</Button>
          </Link>
        }
      />

      <div className="mb-4 grid gap-4 sm:grid-cols-2 lg:grid-cols-5">
        <Select
          label="Status"
          value={statusFilter}
          onChange={(event) => {
            setStatusFilter(event.target.value as SaleStatusFilter);
            setPage(1);
          }}
        >
          {(Object.keys(SALE_STATUS_FILTER_LABELS) as SaleStatusFilter[]).map((value) => (
            <option key={value || 'all'} value={value}>
              {SALE_STATUS_FILTER_LABELS[value]}
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

        <Select
          label="Driver"
          value={driverFilter}
          onChange={(event) => {
            setDriverFilter(event.target.value);
            setPage(1);
          }}
        >
          <option value="">All drivers</option>
          {(drivers.data ?? []).map((driver) => (
            <option key={driver.id} value={driver.id}>
              {driver.name}
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

      {sales.isLoading ? (
        <div className="flex justify-center py-16">
          <LoadingSpinner />
        </div>
      ) : sales.data && sales.data.items.length > 0 ? (
        <DataTable
          caption="Sales"
          columns={columns}
          rows={sales.data.items}
          getRowKey={(row) => row.id}
          pagination={pagination}
          onPageChange={setPage}
          onRowClick={(row) => {
            void navigate({ to: '/sales/$id', params: { id: row.id } });
          }}
        />
      ) : (
        <EmptyState
          title="No sales found"
          description={
            statusFilter || commerceFilter || driverFilter || fromDate || toDate
              ? 'Try adjusting filters or create a new sale.'
              : 'Sales will appear here once recorded in the field.'
          }
        />
      )}
    </div>
  );
}

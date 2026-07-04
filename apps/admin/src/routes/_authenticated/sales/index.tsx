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
import { useI18n } from '@/lib/i18n/context';
import { saleStatusFilterLabel } from '@/lib/i18n/labels';
import { formatMoney } from '@/lib/products/formatPrice';
import { SALE_STATUS_FILTERS, type SaleStatusFilter } from '@/lib/sales/constants';
import { paginatedResponseToTable } from '@/lib/tablePagination';

export const Route = createFileRoute('/_authenticated/sales/')({
  component: SalesListPage,
});

function SalesListPage() {
  const { t } = useI18n();
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

  const columns: DataTableColumn<SaleSummary>[] = useMemo(
    () => [
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
        id: 'commerce',
        header: t('forms.fields.commerce'),
        cell: (row) => commerceNames.get(row.commerceId) ?? row.commerceId.slice(0, 8),
      },
      {
        id: 'driver',
        header: t('forms.fields.driver'),
        cell: (row) => driverNames.get(row.driverId) ?? row.driverId.slice(0, 8),
      },
      {
        id: 'payment',
        header: t('forms.fields.paymentMethod'),
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
        header: t('common.table.total'),
        align: 'right',
        cell: (row) => formatMoney(row.totalAmount, row.totalCurrency),
      },
    ],
    [t, commerceNames, driverNames],
  );

  return (
    <div>
      <PageHeader
        title={t('sales.list.title')}
        description={t('sales.list.description')}
        actions={
          <Link to="/sales/new">
            <Button>{t('sales.list.newSale')}</Button>
          </Link>
        }
      />

      <div className="mb-4 grid gap-4 sm:grid-cols-2 lg:grid-cols-5">
        <Select
          label={t('forms.fields.status')}
          value={statusFilter}
          onChange={(event) => {
            setStatusFilter(event.target.value as SaleStatusFilter);
            setPage(1);
          }}
        >
          {SALE_STATUS_FILTERS.map((value) => (
            <option key={value || 'all'} value={value}>
              {saleStatusFilterLabel(t, value)}
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

        <Select
          label={t('forms.fields.driver')}
          value={driverFilter}
          onChange={(event) => {
            setDriverFilter(event.target.value);
            setPage(1);
          }}
        >
          <option value="">{t('common.filter.allDrivers')}</option>
          {(drivers.data ?? []).map((driver) => (
            <option key={driver.id} value={driver.id}>
              {driver.name}
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

      {sales.isLoading ? (
        <div className="flex justify-center py-16">
          <LoadingSpinner />
        </div>
      ) : sales.data && sales.data.items.length > 0 ? (
        <DataTable
          caption={t('sales.list.caption')}
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
          title={t('sales.list.empty.title')}
          description={
            statusFilter || commerceFilter || driverFilter || fromDate || toDate
              ? t('sales.list.empty.descriptionFiltered')
              : t('sales.list.empty.descriptionDefault')
          }
        />
      )}
    </div>
  );
}

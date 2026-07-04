import { Link, createFileRoute, useNavigate } from '@tanstack/react-router';
import { useQuery } from '@tanstack/react-query';
import { useMemo, useState } from 'react';

import { DomainStatusBadge } from '@/components/status/DomainStatusBadge';
import { DataTable, type DataTableColumn } from '@/components/ui/DataTable';
import { EmptyState } from '@/components/ui/EmptyState';
import { LoadingSpinner } from '@/components/ui/LoadingSpinner';
import { PageHeader } from '@/components/ui/PageHeader';
import { Select } from '@/components/ui/Select';
import { fetchDeliveries } from '@/lib/api/deliveries';
import { fetchDriversForPicker } from '@/lib/api/users';
import type { DeliveryDetail } from '@/lib/api/types';
import { DELIVERY_STATUS_FILTERS, type DeliveryStatusFilter } from '@/lib/deliveries/constants';
import { getDeliveryStatusToken, type DeliveryStatus } from '@/lib/admin-tokens';
import { useI18n } from '@/lib/i18n/context';
import { deliveryStatusFilterLabel, translateDeliveryStatus } from '@/lib/i18n/labels';
import { paginatedResponseToTable } from '@/lib/tablePagination';

export const Route = createFileRoute('/_authenticated/deliveries/')({
  component: DeliveriesListPage,
});

function DeliveriesListPage() {
  const { t } = useI18n();
  const navigate = useNavigate();
  const [page, setPage] = useState(1);
  const [statusFilter, setStatusFilter] = useState<DeliveryStatusFilter>('');
  const pageSize = 20;

  const deliveries = useQuery({
    queryKey: ['deliveries', page, pageSize, statusFilter],
    queryFn: () => fetchDeliveries({ page, pageSize, status: statusFilter }),
  });

  const drivers = useQuery({
    queryKey: ['users', 'drivers', 'picker'],
    queryFn: fetchDriversForPicker,
  });

  const driverNames = useMemo(() => {
    const map = new Map<string, string>();
    for (const driver of drivers.data ?? []) {
      map.set(driver.id, driver.name);
    }
    return map;
  }, [drivers.data]);

  const pagination = deliveries.data ? paginatedResponseToTable(deliveries.data) : null;

  const columns: DataTableColumn<DeliveryDetail>[] = useMemo(
    () => [
      {
        id: 'id',
        header: t('forms.fields.delivery'),
        cell: (row) => <span className="font-mono text-xs">{row.id.slice(0, 8)}…</span>,
      },
      {
        id: 'orderId',
        header: t('forms.fields.order'),
        cell: (row) => (
          <Link
            to="/orders/$id"
            params={{ id: row.orderId }}
            className="font-mono text-xs hover:underline"
            onClick={(event) => {
              event.stopPropagation();
            }}
          >
            {row.orderId.slice(0, 8)}…
          </Link>
        ),
      },
      {
        id: 'status',
        header: t('common.table.status'),
        cell: (row) => (
          <DomainStatusBadge
            colors={getDeliveryStatusToken(row.status as DeliveryStatus)}
            label={translateDeliveryStatus(t, row.status as DeliveryStatus)}
          />
        ),
      },
      {
        id: 'driverId',
        header: t('forms.fields.driver'),
        cell: (row) => driverNames.get(row.driverId) ?? `${row.driverId.slice(0, 8)}…`,
      },
    ],
    [t, driverNames],
  );

  return (
    <div>
      <PageHeader
        title={t('deliveries.list.title')}
        description={t('deliveries.list.description')}
      />

      <div className="mb-4 max-w-xs">
        <Select
          label={t('deliveries.list.filterByStatus')}
          value={statusFilter}
          onChange={(event) => {
            setStatusFilter(event.target.value as DeliveryStatusFilter);
            setPage(1);
          }}
        >
          {DELIVERY_STATUS_FILTERS.map((value) => (
            <option key={value || 'all'} value={value}>
              {deliveryStatusFilterLabel(t, value)}
            </option>
          ))}
        </Select>
      </div>

      {deliveries.isLoading ? (
        <div className="flex justify-center py-16">
          <LoadingSpinner />
        </div>
      ) : deliveries.data && deliveries.data.items.length > 0 ? (
        <DataTable
          caption={t('deliveries.list.caption')}
          columns={columns}
          rows={deliveries.data.items}
          getRowKey={(row) => row.id}
          pagination={pagination}
          onPageChange={setPage}
          onRowClick={(row) => {
            void navigate({ to: '/deliveries/$id', params: { id: row.id } });
          }}
        />
      ) : (
        <EmptyState
          title={t('deliveries.list.empty.title')}
          description={t('deliveries.list.empty.description')}
        />
      )}
    </div>
  );
}

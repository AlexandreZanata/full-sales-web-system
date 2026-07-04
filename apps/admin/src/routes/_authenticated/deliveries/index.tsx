import { Link, createFileRoute, useNavigate } from '@tanstack/react-router';
import { useQuery } from '@tanstack/react-query';
import { useState } from 'react';

import { DomainStatusBadge } from '@/components/status/DomainStatusBadge';
import { DataTable, type DataTableColumn } from '@/components/ui/DataTable';
import { EmptyState } from '@/components/ui/EmptyState';
import { LoadingSpinner } from '@/components/ui/LoadingSpinner';
import { PageHeader } from '@/components/ui/PageHeader';
import { Select } from '@/components/ui/Select';
import { fetchDeliveries } from '@/lib/api/deliveries';
import type { DeliveryDetail } from '@/lib/api/types';
import {
  DELIVERY_STATUS_FILTER_LABELS,
  type DeliveryStatusFilter,
} from '@/lib/deliveries/constants';
import { getDeliveryStatusToken, type DeliveryStatus } from '@/lib/admin-tokens';
import { paginatedResponseToTable } from '@/lib/tablePagination';

export const Route = createFileRoute('/_authenticated/deliveries/')({
  component: DeliveriesListPage,
});

function DeliveriesListPage() {
  const navigate = useNavigate();
  const [page, setPage] = useState(1);
  const [statusFilter, setStatusFilter] = useState<DeliveryStatusFilter>('');
  const pageSize = 20;

  const deliveries = useQuery({
    queryKey: ['deliveries', page, pageSize, statusFilter],
    queryFn: () => fetchDeliveries({ page, pageSize, status: statusFilter }),
  });

  const pagination = deliveries.data ? paginatedResponseToTable(deliveries.data) : null;

  const columns: DataTableColumn<DeliveryDetail>[] = [
    {
      id: 'id',
      header: 'Delivery',
      cell: (row) => <span className="font-mono text-xs">{row.id.slice(0, 8)}…</span>,
    },
    {
      id: 'orderId',
      header: 'Order',
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
      header: 'Status',
      cell: (row) => (
        <DomainStatusBadge colors={getDeliveryStatusToken(row.status as DeliveryStatus)} />
      ),
    },
    {
      id: 'driverId',
      header: 'Driver',
      cell: (row) => <span className="font-mono text-xs">{row.driverId.slice(0, 8)}…</span>,
    },
  ];

  return (
    <div>
      <PageHeader
        title="Deliveries"
        description="Monitor all deliveries across the fleet (read-only oversight)."
      />

      <div className="mb-4 max-w-xs">
        <Select
          label="Filter by status"
          value={statusFilter}
          onChange={(event) => {
            setStatusFilter(event.target.value as DeliveryStatusFilter);
            setPage(1);
          }}
        >
          {(Object.keys(DELIVERY_STATUS_FILTER_LABELS) as DeliveryStatusFilter[]).map((value) => (
            <option key={value || 'all'} value={value}>
              {DELIVERY_STATUS_FILTER_LABELS[value]}
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
          caption="Deliveries"
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
          title="No deliveries found"
          description={
            statusFilter
              ? 'Try another status filter or assign a delivery from an order.'
              : 'Deliveries appear here once assigned from approved or picking orders.'
          }
        />
      )}
    </div>
  );
}

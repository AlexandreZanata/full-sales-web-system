import { Link, createFileRoute } from '@tanstack/react-router';
import { useQuery } from '@tanstack/react-query';
import { useMemo, useState } from 'react';

import { ActiveBadge } from '@/components/users/ActiveBadge';
import { Button } from '@/components/ui/Button';
import { DataTable, type DataTableColumn } from '@/components/ui/DataTable';
import { EmptyState } from '@/components/ui/EmptyState';
import { LoadingSpinner } from '@/components/ui/LoadingSpinner';
import { PageHeader } from '@/components/ui/PageHeader';
import { Select } from '@/components/ui/Select';
import { fetchCommerces } from '@/lib/api/commerces';
import type { CommerceSummary } from '@/lib/api/types';
import { ACTIVE_FILTERS, type ActiveFilter } from '@/lib/commerces/constants';
import { formatCnpj } from '@/lib/commerces/cnpj';
import { useI18n } from '@/lib/i18n/context';
import { activeFilterLabel } from '@/lib/i18n/labels';
import { paginatedResponseToTable } from '@/lib/tablePagination';

export const Route = createFileRoute('/_authenticated/commerces/')({
  component: CommercesListPage,
});

function CommercesListPage() {
  const { t } = useI18n();
  const [page, setPage] = useState(1);
  const [activeFilter, setActiveFilter] = useState<ActiveFilter>('');
  const pageSize = 20;

  const commerces = useQuery({
    queryKey: ['commerces', page, pageSize, activeFilter],
    queryFn: () => fetchCommerces({ page, pageSize, active: activeFilter }),
  });

  const pagination = commerces.data ? paginatedResponseToTable(commerces.data) : null;

  const columns: DataTableColumn<CommerceSummary>[] = useMemo(
    () => [
      {
        id: 'cnpj',
        header: t('forms.fields.cnpj'),
        cell: (row) => formatCnpj(row.cnpj),
      },
      {
        id: 'tradeName',
        header: t('forms.fields.tradeName'),
        cell: (row) => (
          <Link to="/commerces/$id" params={{ id: row.id }} className="font-medium hover:underline">
            {row.tradeName || row.legalName}
          </Link>
        ),
      },
      {
        id: 'legalName',
        header: t('forms.fields.legalName'),
        cell: (row) => row.legalName,
      },
      {
        id: 'active',
        header: t('forms.fields.status'),
        cell: (row) => <ActiveBadge active={row.active} />,
      },
    ],
    [t],
  );

  return (
    <div>
      <PageHeader
        title={t('commerces.list.title')}
        description={t('commerces.list.description')}
        actions={
          <Link to="/commerces/new">
            <Button>{t('commerces.list.register')}</Button>
          </Link>
        }
      />

      <div className="mb-4 max-w-xs">
        <Select
          label={t('commerces.list.filterByStatus')}
          value={activeFilter}
          onChange={(event) => {
            setActiveFilter(event.target.value as ActiveFilter);
            setPage(1);
          }}
        >
          {ACTIVE_FILTERS.map((value) => (
            <option key={value || 'all'} value={value}>
              {activeFilterLabel(t, value)}
            </option>
          ))}
        </Select>
      </div>

      {commerces.isLoading ? (
        <div className="flex justify-center py-16">
          <LoadingSpinner />
        </div>
      ) : commerces.data && commerces.data.items.length > 0 ? (
        <DataTable
          caption={t('commerces.list.caption')}
          columns={columns}
          rows={commerces.data.items}
          getRowKey={(row) => row.id}
          pagination={pagination}
          onPageChange={setPage}
        />
      ) : (
        <EmptyState
          title={t('commerces.list.empty.title')}
          description={
            activeFilter
              ? t('commerces.list.empty.descriptionFiltered')
              : t('commerces.list.empty.descriptionDefault')
          }
        />
      )}
    </div>
  );
}

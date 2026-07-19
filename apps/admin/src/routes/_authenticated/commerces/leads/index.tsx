import { createFileRoute, Link } from '@tanstack/react-router';
import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';
import { useMemo, useState } from 'react';

import { DataTable, type DataTableColumn } from '@/components/ui/DataTable';
import { EmptyState } from '@/components/ui/EmptyState';
import { LoadingSpinner } from '@/components/ui/LoadingSpinner';
import { PageHeader } from '@/components/ui/PageHeader';
import { Select } from '@/components/ui/Select';
import { TableActions, tableActionClass } from '@/components/ui/TableActions';
import { useToast } from '@/hooks/useToast';
import {
  fetchPortalLeads,
  reviewPortalLead,
  type PortalLead,
  type PortalLeadStatus,
} from '@/lib/api/portalLeads';
import { useI18n } from '@/lib/i18n/context';

export const Route = createFileRoute('/_authenticated/commerces/leads/')({
  component: PortalLeadsPage,
});

function PortalLeadsPage() {
  const { t } = useI18n();
  const toast = useToast();
  const queryClient = useQueryClient();
  const [statusFilter, setStatusFilter] = useState<PortalLeadStatus | ''>('pending');

  const leads = useQuery({
    queryKey: ['portal-leads', statusFilter],
    queryFn: () => fetchPortalLeads(statusFilter || undefined),
  });

  const review = useMutation({
    mutationFn: ({ id, status }: { id: string; status: 'approved' | 'rejected' }) =>
      reviewPortalLead(id, status),
    onSuccess: (_data, vars) => {
      toast.success(
        vars.status === 'approved'
          ? t('commerces.leads.toast.approved')
          : t('commerces.leads.toast.rejected'),
      );
      void queryClient.invalidateQueries({ queryKey: ['portal-leads'] });
    },
    onError: (err: Error) => toast.error(err.message),
  });

  const columns = useMemo((): DataTableColumn<PortalLead>[] => {
    return [
      {
        id: 'contact',
        header: t('commerces.leads.columns.contactName'),
        cell: (row) => row.contactName,
      },
      {
        id: 'commerce',
        header: t('commerces.leads.columns.commerceName'),
        cell: (row) => row.commerceName,
      },
      {
        id: 'phone',
        header: t('commerces.leads.columns.phone'),
        cell: (row) => row.phone,
      },
      {
        id: 'email',
        header: t('commerces.leads.columns.email'),
        cell: (row) => row.email,
      },
      {
        id: 'status',
        header: t('commerces.leads.columns.status'),
        cell: (row) => t(`commerces.leads.status.${row.status}`),
      },
      {
        id: 'created',
        header: t('commerces.leads.columns.createdAt'),
        cell: (row) => new Date(row.createdAt).toLocaleString(),
      },
      {
        id: 'actions',
        header: '',
        cell: (row) =>
          row.status === 'pending' ? (
            <TableActions>
              <button
                type="button"
                className={tableActionClass('success')}
                disabled={review.isPending}
                onClick={() => review.mutate({ id: row.id, status: 'approved' })}
              >
                {t('commerces.leads.approve')}
              </button>
              <button
                type="button"
                className={tableActionClass('danger')}
                disabled={review.isPending}
                onClick={() => review.mutate({ id: row.id, status: 'rejected' })}
              >
                {t('commerces.leads.reject')}
              </button>
            </TableActions>
          ) : null,
      },
    ];
  }, [review, t]);

  return (
    <div>
      <PageHeader
        title={t('commerces.leads.title')}
        description={t('commerces.leads.description')}
        back={<Link to="/commerces">{t('common.backTo.commerces')}</Link>}
      />
      <div className="mb-4 max-w-xs">
        <Select
          label={t('commerces.leads.filterStatus')}
          value={statusFilter}
          onChange={(event) => {
            setStatusFilter(event.target.value as PortalLeadStatus | '');
          }}
        >
          <option value="pending">{t('commerces.leads.status.pending')}</option>
          <option value="approved">{t('commerces.leads.status.approved')}</option>
          <option value="rejected">{t('commerces.leads.status.rejected')}</option>
          <option value="">{t('commerces.leads.status.all')}</option>
        </Select>
      </div>
      {leads.isLoading ? <LoadingSpinner /> : null}
      {leads.isError ? <p className="text-sm text-destructive">{leads.error.message}</p> : null}
      {leads.data && leads.data.length === 0 ? (
        <EmptyState
          title={t('commerces.leads.empty.title')}
          description={t('commerces.leads.empty.description')}
        />
      ) : null}
      {leads.data && leads.data.length > 0 ? (
        <DataTable
          columns={columns}
          rows={leads.data}
          getRowKey={(row) => row.id}
          searchable
          getSearchText={(row) =>
            `${row.contactName} ${row.commerceName} ${row.email} ${row.phone}`
          }
        />
      ) : null}
    </div>
  );
}

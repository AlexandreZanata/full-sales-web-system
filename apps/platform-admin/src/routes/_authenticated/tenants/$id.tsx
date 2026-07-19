import { createFileRoute } from '@tanstack/react-router';
import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';
import { useState } from 'react';

import { Button } from '@/components/ui/Button';
import { Card } from '@/components/ui/Card';
import { ConfirmDialog } from '@/components/ui/ConfirmDialog';
import { DataTable, type DataTableColumn } from '@/components/ui/DataTable';
import { EmptyState } from '@/components/ui/EmptyState';
import { LoadingSpinner } from '@/components/ui/LoadingSpinner';
import { PageBackLink } from '@/components/ui/PageBackLink';
import { PageHeader } from '@/components/ui/PageHeader';
import { Input } from '@/components/ui/Input';
import { JsonBlock } from '@/components/ui/JsonBlock';
import { Tabs } from '@/components/ui/Tabs';
import { Textarea } from '@/components/ui/Textarea';
import { fetchPlatformAuditEvents } from '@/lib/api/audit';
import { fetchPlatformDomains } from '@/lib/api/domains';
import {
  fetchTenant,
  offboardTenant,
  patchTenant,
  reactivateTenant,
  suspendTenant,
} from '@/lib/api/tenants';
import { fetchTenantStats, fetchTenantWorkforce } from '@/lib/api/users';
import type { PlatformUser } from '@/lib/api/types';
import { formatDateTime } from '@/lib/formatDateTime';
import { formatMoneyMinor } from '@/lib/i18n/labels';
import { useI18n } from '@/lib/i18n/context';
import { useToast } from '@/hooks/useToast';

export const Route = createFileRoute('/_authenticated/tenants/$id')({
  component: TenantDetailPage,
});

function TenantDetailPage() {
  const { id } = Route.useParams();
  const { t } = useI18n();
  const toast = useToast();
  const queryClient = useQueryClient();
  const [tab, setTab] = useState('overview');
  const [suspendOpen, setSuspendOpen] = useState(false);
  const [reason, setReason] = useState('');
  const [planId, setPlanId] = useState('');

  const tenant = useQuery({ queryKey: ['tenant', id], queryFn: () => fetchTenant(id) });
  const stats = useQuery({
    queryKey: ['tenant', id, 'stats'],
    queryFn: () => fetchTenantStats(id),
  });
  const workforce = useQuery({
    queryKey: ['tenant', id, 'workforce'],
    queryFn: () => fetchTenantWorkforce(id, { limit: 50 }),
    enabled: tab === 'users',
  });
  const domains = useQuery({
    queryKey: ['tenant', id, 'domains'],
    queryFn: () => fetchPlatformDomains({ limit: 50 }),
    enabled: tab === 'domains',
  });
  const audit = useQuery({
    queryKey: ['tenant', id, 'audit'],
    queryFn: () => fetchPlatformAuditEvents({ limit: 20, tenantId: id }),
    enabled: tab === 'audit',
  });

  const suspend = useMutation({
    mutationFn: () => suspendTenant(id, reason),
    onSuccess: () => {
      void queryClient.invalidateQueries({ queryKey: ['tenant', id] });
      setSuspendOpen(false);
    },
  });
  const reactivate = useMutation({
    mutationFn: () => reactivateTenant(id),
    onSuccess: () => void queryClient.invalidateQueries({ queryKey: ['tenant', id] }),
  });
  const changePlan = useMutation({
    mutationFn: () => patchTenant(id, { planId }),
    onSuccess: () => {
      toast.success('Plan updated');
      void queryClient.invalidateQueries({ queryKey: ['tenant', id] });
    },
  });
  const offboard = useMutation({
    mutationFn: () => offboardTenant(id),
    onSuccess: () => {
      toast.success('Offboarding scheduled');
      void queryClient.invalidateQueries({ queryKey: ['tenant', id] });
    },
  });

  if (tenant.isLoading) {
    return <LoadingSpinner />;
  }
  if (!tenant.data) {
    return <EmptyState title={t('common.noResults')} />;
  }

  const userColumns: DataTableColumn<PlatformUser>[] = [
    { id: 'name', header: t('common.name'), cell: (row) => row.name },
    { id: 'email', header: t('common.email'), cell: (row) => row.email },
    { id: 'role', header: 'Role', cell: (row) => row.role },
  ];

  const tabItems = [
    { id: 'overview', label: t('tenants.overview') },
    { id: 'users', label: t('tenants.workforce') },
    { id: 'billing', label: t('tenants.billing') },
    { id: 'settings', label: t('tenants.settings') },
    { id: 'domains', label: t('tenants.domains') },
    { id: 'audit', label: t('tenants.audit') },
  ];

  return (
    <div className="space-y-4">
      <PageHeader
        title={tenant.data.displayName}
        back={<PageBackLink to="/tenants" label={t('tenants.title')} />}
      />
      <div className="flex flex-wrap gap-2">
        <Button
          variant="secondary"
          onClick={() => {
            setSuspendOpen(true);
          }}
        >
          {t('tenants.suspend')}
        </Button>
        <Button
          variant="secondary"
          onClick={() => {
            reactivate.mutate();
          }}
        >
          {t('tenants.reactivate')}
        </Button>
        <Button
          variant="danger"
          onClick={() => {
            offboard.mutate();
          }}
        >
          {t('tenants.offboard')}
        </Button>
      </div>
      <Tabs tabs={tabItems} activeId={tab} onChange={setTab}>
        {tab === 'overview' ? (
          <Card className="grid gap-2 p-4 text-sm md:grid-cols-2">
            <p>
              <strong>{t('common.status')}:</strong> {tenant.data.status}
            </p>
            <p>
              <strong>Users:</strong> {tenant.data.counts.users}
            </p>
            <p>
              <strong>Orders:</strong> {tenant.data.counts.orders}
            </p>
          </Card>
        ) : null}
        {tab === 'billing' && stats.data ? (
          <Card className="space-y-3 p-4 text-sm">
            <p>MRR: {formatMoneyMinor(stats.data.mrrMinor, stats.data.mrrCurrency)}</p>
            <p>Orders: {stats.data.orders}</p>
            <div className="flex flex-wrap items-end gap-2">
              <Input
                label={t('tenants.changePlan')}
                value={planId || tenant.data.planId || ''}
                onChange={(e) => {
                  setPlanId(e.target.value);
                }}
              />
              <Button
                onClick={() => {
                  changePlan.mutate();
                }}
                disabled={!planId.trim()}
              >
                {t('common.save')}
              </Button>
            </div>
          </Card>
        ) : null}
        {tab === 'settings' ? <JsonBlock value={tenant.data.settings} /> : null}
        {tab === 'users' && workforce.data ? (
          <DataTable columns={userColumns} rows={workforce.data.data} getRowKey={(row) => row.id} />
        ) : null}
        {tab === 'domains' && domains.data ? (
          <ul className="space-y-2 text-sm">
            {domains.data.data
              .filter((d) => d.tenantId === id)
              .map((d) => (
                <li key={d.id} className="rounded border border-hairline p-3">
                  {d.hostname} — {d.status}
                </li>
              ))}
          </ul>
        ) : null}
        {tab === 'audit' && audit.data ? (
          <ul className="space-y-2 text-sm">
            {audit.data.data.map((event) => (
              <li key={event.id} className="rounded border border-hairline p-3">
                {event.action} · {formatDateTime(event.createdAt)}
              </li>
            ))}
          </ul>
        ) : null}
      </Tabs>
      <ConfirmDialog
        open={suspendOpen}
        title={t('tenants.suspend')}
        message={t('tenants.suspendReason')}
        confirmDisabled={reason.trim().length < 3}
        onCancel={() => {
          setSuspendOpen(false);
          setReason('');
        }}
        onConfirm={() => {
          suspend.mutate();
        }}
      >
        <Textarea
          label={t('tenants.suspendReason')}
          value={reason}
          onChange={(e) => {
            setReason(e.target.value);
          }}
        />
      </ConfirmDialog>
    </div>
  );
}

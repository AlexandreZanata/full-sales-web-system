import { Link, createFileRoute } from '@tanstack/react-router';
import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';
import { useState } from 'react';

import { TenantDetailPanels } from '@/components/tenants/TenantDetailPanels';
import { Button } from '@/components/ui/Button';
import { ConfirmDialog } from '@/components/ui/ConfirmDialog';
import { EmptyState } from '@/components/ui/EmptyState';
import { LoadingSpinner } from '@/components/ui/LoadingSpinner';
import { PageBackLink } from '@/components/ui/PageBackLink';
import { PageHeader } from '@/components/ui/PageHeader';
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
        actions={
          <Link to="/tenants" search={{ modal: 'edit', id }}>
            <Button variant="secondary">{t('tenants.edit')}</Button>
          </Link>
        }
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
        <TenantDetailPanels
          tab={tab}
          tenantId={id}
          tenant={tenant.data}
          planId={planId}
          onPlanIdChange={setPlanId}
          onChangePlan={() => {
            changePlan.mutate();
          }}
          stats={stats.data}
          workforce={workforce.data}
          domains={domains.data}
          audit={audit.data}
        />
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

import { createFileRoute } from '@tanstack/react-router';
import { useQuery, useQueryClient } from '@tanstack/react-query';
import { useMemo, useState, type ReactNode } from 'react';

import { AddressesTab } from '@/components/commerces/AddressesTab';
import { CommerceLogoSection } from '@/components/commerces/CommerceLogoSection';
import { ActiveBadge } from '@/components/users/ActiveBadge';
import { Button } from '@/components/ui/Button';
import { Card } from '@/components/ui/Card';
import { ConfirmDialog } from '@/components/ui/ConfirmDialog';
import { LoadingSpinner } from '@/components/ui/LoadingSpinner';
import { PageBackLink } from '@/components/ui/PageBackLink';
import { PageHeader } from '@/components/ui/PageHeader';
import { Tabs } from '@/components/ui/Tabs';
import { useToast } from '@/hooks/useToast';
import {
  activateCommerce,
  deactivateCommerce,
  fetchCommerce,
  fetchCommerceAddresses,
} from '@/lib/api/commerces';
import { formatCnpj } from '@/lib/commerces/cnpj';
import { useI18n } from '@/lib/i18n/context';

export const Route = createFileRoute('/_authenticated/commerces/$id')({
  component: CommerceDetailPage,
});

function CommerceDetailPage() {
  const { id } = Route.useParams();
  const { t } = useI18n();
  const queryClient = useQueryClient();
  const toast = useToast();
  const [activeTab, setActiveTab] = useState('overview');
  const [confirmOpen, setConfirmOpen] = useState(false);
  const [busy, setBusy] = useState(false);

  const commerce = useQuery({
    queryKey: ['commerces', id],
    queryFn: () => fetchCommerce(id),
  });

  const addresses = useQuery({
    queryKey: ['commerces', id, 'addresses'],
    queryFn: () => fetchCommerceAddresses(id),
    enabled: activeTab === 'addresses',
  });

  const tabs = useMemo(
    () => [
      { id: 'overview', label: t('commerces.detail.tabs.overview') },
      { id: 'addresses', label: t('commerces.detail.tabs.addresses') },
    ],
    [t],
  );

  async function invalidateCommerce() {
    await queryClient.invalidateQueries({ queryKey: ['commerces'] });
    await queryClient.invalidateQueries({ queryKey: ['commerces', id] });
  }

  async function handleDeactivate() {
    setBusy(true);
    try {
      await deactivateCommerce(id);
      await invalidateCommerce();
      toast.success(t('commerces.toast.deactivated'));
      setConfirmOpen(false);
    } catch {
      toast.error(t('errors.actionFailed'));
    } finally {
      setBusy(false);
    }
  }

  async function handleReactivate() {
    setBusy(true);
    try {
      await activateCommerce(id);
      await invalidateCommerce();
      toast.success(t('commerces.toast.reactivated'));
    } catch {
      toast.error(t('errors.actionFailed'));
    } finally {
      setBusy(false);
    }
  }

  if (commerce.isLoading) {
    return (
      <div className="flex justify-center py-16">
        <LoadingSpinner />
      </div>
    );
  }

  if (!commerce.data) {
    return (
      <PageHeader
        title={t('commerces.detail.notFound')}
        back={<PageBackLink label={t('common.backTo.commerces')} to="/commerces" />}
      />
    );
  }

  const detail = commerce.data;

  return (
    <div>
      <PageHeader
        title={detail.tradeName || detail.legalName}
        description={formatCnpj(detail.cnpj)}
        back={<PageBackLink label={t('common.backTo.commerces')} to="/commerces" />}
        actions={
          detail.active ? (
            <Button
              variant="danger"
              disabled={busy}
              onClick={() => {
                setConfirmOpen(true);
              }}
            >
              {t('commerces.detail.deactivate')}
            </Button>
          ) : (
            <Button variant="success" disabled={busy} onClick={() => void handleReactivate()}>
              {t('commerces.detail.reactivate')}
            </Button>
          )
        }
      />

      <Tabs tabs={tabs} activeId={activeTab} onChange={setActiveTab}>
        {activeTab === 'overview' ? (
          <div className="space-y-4">
            <Card className="space-y-3">
              <DetailRow label={t('forms.fields.cnpj')} value={formatCnpj(detail.cnpj)} />
              <DetailRow label={t('forms.fields.legalName')} value={detail.legalName} />
              <DetailRow label={t('forms.fields.tradeName')} value={detail.tradeName || '—'} />
              <DetailRow
                label={t('forms.fields.status')}
                value={<ActiveBadge active={detail.active} />}
              />
            </Card>
            <Card>
              <CommerceLogoSection commerceId={id} logoFileId={detail.logoFileId} />
            </Card>
          </div>
        ) : null}

        {activeTab === 'addresses' ? (
          addresses.isLoading ? (
            <div className="flex justify-center py-16">
              <LoadingSpinner />
            </div>
          ) : (
            <AddressesTab
              commerceId={id}
              addresses={addresses.data ?? []}
              onChanged={() => {
                void queryClient.invalidateQueries({ queryKey: ['commerces', id, 'addresses'] });
              }}
            />
          )
        ) : null}
      </Tabs>

      <ConfirmDialog
        open={confirmOpen}
        title={t('commerces.detail.deactivateDialog.title')}
        message={t('commerces.detail.deactivateDialog.message')}
        confirmLabel={t('commerces.detail.deactivateDialog.confirm')}
        destructive
        isLoading={busy}
        onCancel={() => {
          setConfirmOpen(false);
        }}
        onConfirm={() => void handleDeactivate()}
      />
    </div>
  );
}

function DetailRow({ label, value }: { label: string; value: ReactNode }) {
  return (
    <div className="flex flex-col gap-1 sm:flex-row sm:items-center sm:justify-between">
      <span className="text-xs font-semibold uppercase tracking-[0.12em] text-muted-foreground">
        {label}
      </span>
      <span className="text-sm text-foreground">{value}</span>
    </div>
  );
}

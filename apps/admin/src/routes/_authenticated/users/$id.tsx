import { createFileRoute } from '@tanstack/react-router';
import { useQuery, useQueryClient } from '@tanstack/react-query';
import { useMemo, useState, type ReactNode } from 'react';

import { ActiveBadge } from '@/components/users/ActiveBadge';
import { DriverProfileTab } from '@/components/users/DriverProfileTab';
import { SellerProfileTab } from '@/components/users/SellerProfileTab';
import { Button } from '@/components/ui/Button';
import { Card } from '@/components/ui/Card';
import { ConfirmDialog } from '@/components/ui/ConfirmDialog';
import { LoadingSpinner } from '@/components/ui/LoadingSpinner';
import { PageBackLink } from '@/components/ui/PageBackLink';
import { PageHeader } from '@/components/ui/PageHeader';
import { Tabs } from '@/components/ui/Tabs';
import { useToast } from '@/hooks/useToast';
import { deactivateUser, fetchUser, reactivateUser } from '@/lib/api/users';
import { useI18n } from '@/lib/i18n/context';
import { translateRole } from '@/lib/i18n/labels';

export const Route = createFileRoute('/_authenticated/users/$id')({
  component: UserDetailPage,
});

function UserDetailPage() {
  const { id } = Route.useParams();
  const { t } = useI18n();
  const queryClient = useQueryClient();
  const toast = useToast();
  const [activeTab, setActiveTab] = useState('overview');
  const [confirmKind, setConfirmKind] = useState<'deactivate' | 'reactivate' | null>(null);
  const [busy, setBusy] = useState(false);

  const user = useQuery({
    queryKey: ['users', id],
    queryFn: () => fetchUser(id),
  });

  const tabs = useMemo(() => {
    const items = [{ id: 'overview', label: t('users.detail.tabs.overview') }];
    if (user.data?.role === 'Driver') {
      items.push({ id: 'driver', label: t('users.detail.tabs.driverProfile') });
    }
    if (user.data?.role === 'Seller') {
      items.push({ id: 'seller', label: t('users.detail.tabs.sellerProfile') });
    }
    return items;
  }, [t, user.data?.role]);

  async function handleConfirm() {
    if (!confirmKind) {
      return;
    }
    setBusy(true);
    try {
      if (confirmKind === 'deactivate') {
        await deactivateUser(id);
        toast.success(t('users.toast.deactivated'));
      } else {
        await reactivateUser(id);
        toast.success(t('users.toast.reactivated'));
      }
      await queryClient.invalidateQueries({ queryKey: ['users'] });
      await queryClient.invalidateQueries({ queryKey: ['users', id] });
      setConfirmKind(null);
    } catch {
      toast.error(t('errors.actionFailed'));
    } finally {
      setBusy(false);
    }
  }

  if (user.isLoading) {
    return (
      <div className="flex justify-center py-16">
        <LoadingSpinner />
      </div>
    );
  }

  if (!user.data) {
    return (
      <PageHeader
        title={t('users.detail.notFound')}
        back={<PageBackLink label={t('common.backTo.users')} to="/users" />}
      />
    );
  }

  const detail = user.data;
  const isDeactivate = confirmKind === 'deactivate';

  return (
    <div>
      <PageHeader
        title={detail.name}
        description={detail.email}
        back={<PageBackLink label={t('common.backTo.users')} to="/users" />}
        actions={
          detail.active ? (
            <Button
              variant="danger"
              onClick={() => {
                setConfirmKind('deactivate');
              }}
            >
              {t('users.detail.deactivate')}
            </Button>
          ) : (
            <Button
              onClick={() => {
                setConfirmKind('reactivate');
              }}
            >
              {t('users.detail.reactivate')}
            </Button>
          )
        }
      />

      <Tabs tabs={tabs} activeId={activeTab} onChange={setActiveTab}>
        {activeTab === 'overview' ? (
          <Card className="space-y-3">
            <DetailRow label={t('forms.fields.name')} value={detail.name} />
            <DetailRow label={t('forms.fields.email')} value={detail.email} />
            <DetailRow label={t('forms.fields.role')} value={translateRole(t, detail.role)} />
            <DetailRow
              label={t('forms.fields.status')}
              value={<ActiveBadge active={detail.active} />}
            />
            {detail.commerceId ? (
              <DetailRow label={t('forms.fields.commerceId')} value={detail.commerceId} />
            ) : null}
          </Card>
        ) : null}

        {activeTab === 'driver' ? <DriverProfileTab userId={id} /> : null}
        {activeTab === 'seller' ? <SellerProfileTab userId={id} /> : null}
      </Tabs>

      <ConfirmDialog
        open={confirmKind !== null}
        title={
          isDeactivate
            ? t('users.detail.deactivateDialog.title')
            : t('users.detail.reactivateDialog.title')
        }
        message={
          isDeactivate
            ? t('users.detail.deactivateDialog.message')
            : t('users.detail.reactivateDialog.message')
        }
        confirmLabel={
          isDeactivate
            ? t('users.detail.deactivateDialog.confirm')
            : t('users.detail.reactivateDialog.confirm')
        }
        destructive={isDeactivate}
        isLoading={busy}
        onCancel={() => {
          setConfirmKind(null);
        }}
        onConfirm={() => void handleConfirm()}
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

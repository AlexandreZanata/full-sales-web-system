import { createFileRoute } from '@tanstack/react-router';
import { useQuery, useQueryClient } from '@tanstack/react-query';
import { useState, type ReactNode } from 'react';

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
import { deactivateUser, fetchUser } from '@/lib/api/users';
import { USER_ROLE_LABELS } from '@/lib/users/constants';

export const Route = createFileRoute('/_authenticated/users/$id')({
  component: UserDetailPage,
});

function UserDetailPage() {
  const { id } = Route.useParams();
  const queryClient = useQueryClient();
  const toast = useToast();
  const [activeTab, setActiveTab] = useState('overview');
  const [confirmOpen, setConfirmOpen] = useState(false);
  const [deactivating, setDeactivating] = useState(false);

  const user = useQuery({
    queryKey: ['users', id],
    queryFn: () => fetchUser(id),
  });

  const tabs = [{ id: 'overview', label: 'Overview' }];
  if (user.data?.role === 'Driver') {
    tabs.push({ id: 'driver', label: 'Driver profile' });
  }
  if (user.data?.role === 'Seller') {
    tabs.push({ id: 'seller', label: 'Seller profile' });
  }

  async function handleDeactivate() {
    setDeactivating(true);
    try {
      await deactivateUser(id);
      await queryClient.invalidateQueries({ queryKey: ['users'] });
      await queryClient.invalidateQueries({ queryKey: ['users', id] });
      toast.success('User deactivated');
      setConfirmOpen(false);
    } catch {
      toast.error('Unable to deactivate user');
    } finally {
      setDeactivating(false);
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
        title="User not found"
        back={<PageBackLink label="Back to users" to="/users" />}
      />
    );
  }

  const detail = user.data;

  return (
    <div>
      <PageHeader
        title={detail.name}
        description={detail.email}
        back={<PageBackLink label="Back to users" to="/users" />}
        actions={
          detail.active ? (
            <Button
              variant="danger"
              onClick={() => {
                setConfirmOpen(true);
              }}
            >
              Deactivate
            </Button>
          ) : null
        }
      />

      <Tabs tabs={tabs} activeId={activeTab} onChange={setActiveTab}>
        {activeTab === 'overview' ? (
          <Card className="space-y-3">
            <DetailRow label="Name" value={detail.name} />
            <DetailRow label="Email" value={detail.email} />
            <DetailRow label="Role" value={USER_ROLE_LABELS[detail.role]} />
            <DetailRow label="Status" value={<ActiveBadge active={detail.active} />} />
            {detail.commerceId ? <DetailRow label="Commerce ID" value={detail.commerceId} /> : null}
          </Card>
        ) : null}

        {activeTab === 'driver' ? <DriverProfileTab userId={id} /> : null}
        {activeTab === 'seller' ? <SellerProfileTab userId={id} /> : null}
      </Tabs>

      <ConfirmDialog
        open={confirmOpen}
        title="Deactivate user"
        message="This user will no longer be able to sign in. You can create a new account later if needed."
        confirmLabel="Deactivate"
        destructive
        isLoading={deactivating}
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

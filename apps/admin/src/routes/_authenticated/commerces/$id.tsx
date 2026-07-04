import { createFileRoute } from '@tanstack/react-router';
import { useQuery, useQueryClient } from '@tanstack/react-query';
import { useState, type ReactNode } from 'react';

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
import { deactivateCommerce, fetchCommerce, fetchCommerceAddresses } from '@/lib/api/commerces';
import { formatCnpj } from '@/lib/commerces/cnpj';

export const Route = createFileRoute('/_authenticated/commerces/$id')({
  component: CommerceDetailPage,
});

function CommerceDetailPage() {
  const { id } = Route.useParams();
  const queryClient = useQueryClient();
  const toast = useToast();
  const [activeTab, setActiveTab] = useState('overview');
  const [confirmOpen, setConfirmOpen] = useState(false);
  const [deactivating, setDeactivating] = useState(false);

  const commerce = useQuery({
    queryKey: ['commerces', id],
    queryFn: () => fetchCommerce(id),
  });

  const addresses = useQuery({
    queryKey: ['commerces', id, 'addresses'],
    queryFn: () => fetchCommerceAddresses(id),
    enabled: activeTab === 'addresses',
  });

  const tabs = [
    { id: 'overview', label: 'Overview' },
    { id: 'addresses', label: 'Addresses' },
  ];

  async function handleDeactivate() {
    setDeactivating(true);
    try {
      await deactivateCommerce(id);
      await queryClient.invalidateQueries({ queryKey: ['commerces'] });
      await queryClient.invalidateQueries({ queryKey: ['commerces', id] });
      toast.success('Commerce deactivated');
      setConfirmOpen(false);
    } catch {
      toast.error('Unable to deactivate commerce');
    } finally {
      setDeactivating(false);
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
        title="Commerce not found"
        back={<PageBackLink label="Back to commerces" to="/commerces" />}
      />
    );
  }

  const detail = commerce.data;

  return (
    <div>
      <PageHeader
        title={detail.tradeName || detail.legalName}
        description={formatCnpj(detail.cnpj)}
        back={<PageBackLink label="Back to commerces" to="/commerces" />}
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
          <div className="space-y-4">
            <Card className="space-y-3">
              <DetailRow label="CNPJ" value={formatCnpj(detail.cnpj)} />
              <DetailRow label="Legal name" value={detail.legalName} />
              <DetailRow label="Trade name" value={detail.tradeName || '—'} />
              <DetailRow label="Status" value={<ActiveBadge active={detail.active} />} />
            </Card>
            <Card>
              <CommerceLogoSection commerceId={id} />
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
        title="Deactivate commerce"
        message="Inactive commerces cannot receive new sales. Existing records remain in the system."
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

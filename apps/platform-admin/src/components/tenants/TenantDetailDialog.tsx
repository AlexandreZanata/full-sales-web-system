import { useQuery } from '@tanstack/react-query';

import { Button } from '@/components/ui/Button';
import { Card } from '@/components/ui/Card';
import { Dialog } from '@/components/ui/Dialog';
import { LoadingSpinner } from '@/components/ui/LoadingSpinner';
import { fetchTenant } from '@/lib/api/tenants';
import { useI18n } from '@/lib/i18n/context';

type TenantDetailDialogProps = {
  tenantId: string | null;
  onClose: () => void;
  onEdit: (id: string) => void;
};

export function TenantDetailDialog({ tenantId, onClose, onEdit }: TenantDetailDialogProps) {
  const { t } = useI18n();
  const open = Boolean(tenantId);
  const tenant = useQuery({
    queryKey: ['tenant', tenantId],
    queryFn: () => {
      if (!tenantId) {
        throw new Error('tenantId required');
      }
      return fetchTenant(tenantId);
    },
    enabled: open,
  });

  return (
    <Dialog
      open={open}
      title={tenant.data?.displayName ?? t('tenants.title')}
      onClose={onClose}
      className="max-w-xl"
      footer={
        tenantId ? (
          <Button
            onClick={() => {
              onClose();
              onEdit(tenantId);
            }}
          >
            {t('tenants.edit')}
          </Button>
        ) : null
      }
    >
      {tenant.isLoading || !tenant.data ? (
        <LoadingSpinner />
      ) : (
        <Card className="grid gap-2 border-0 p-0 text-sm shadow-none md:grid-cols-2">
          <p>
            <strong>{t('tenants.legalName')}:</strong> {tenant.data.legalName}
          </p>
          <p>
            <strong>{t('tenants.displayName')}:</strong> {tenant.data.displayName}
          </p>
          <p>
            <strong>{t('common.status')}:</strong> {tenant.data.status}
          </p>
          <p>
            <strong>{t('tenants.planId')}:</strong> {tenant.data.planId ?? '—'}
          </p>
          <p>
            <strong>Users:</strong> {tenant.data.counts.users}
          </p>
          <p>
            <strong>Orders:</strong> {tenant.data.counts.orders}
          </p>
        </Card>
      )}
    </Dialog>
  );
}

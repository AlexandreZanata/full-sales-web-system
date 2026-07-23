import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';
import { useEffect, useState, type SubmitEvent } from 'react';

import { Button } from '@/components/ui/Button';
import { Dialog } from '@/components/ui/Dialog';
import { Input } from '@/components/ui/Input';
import { LoadingSpinner } from '@/components/ui/LoadingSpinner';
import { fetchTenant, patchTenant } from '@/lib/api/tenants';
import { useI18n } from '@/lib/i18n/context';
import { useToast } from '@/hooks/useToast';

type TenantEditDialogProps = {
  tenantId: string | null;
  onClose: () => void;
};

export function TenantEditDialog({ tenantId, onClose }: TenantEditDialogProps) {
  const { t } = useI18n();
  const toast = useToast();
  const queryClient = useQueryClient();
  const [displayName, setDisplayName] = useState('');
  const [planId, setPlanId] = useState('');
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

  useEffect(() => {
    if (!tenant.data) {
      return;
    }
    setDisplayName(tenant.data.displayName);
    setPlanId(tenant.data.planId ?? '');
  }, [tenant.data]);

  const mutation = useMutation({
    mutationFn: () => {
      if (!tenantId) {
        throw new Error('tenantId required');
      }
      return patchTenant(tenantId, {
        displayName: displayName.trim(),
        planId: planId.trim() || undefined,
      });
    },
    onSuccess: async () => {
      toast.success(t('tenants.updated'));
      await queryClient.invalidateQueries({ queryKey: ['tenant', tenantId] });
      await queryClient.invalidateQueries({ queryKey: ['tenants'] });
      onClose();
    },
    onError: () => {
      toast.error(t('common.unexpectedError'));
    },
  });

  function handleSubmit(event: SubmitEvent<HTMLFormElement>) {
    event.preventDefault();
    if (!displayName.trim()) {
      return;
    }
    mutation.mutate();
  }

  return (
    <Dialog
      open={open}
      title={t('tenants.edit')}
      onClose={onClose}
      footer={
        <>
          <Button type="button" variant="secondary" onClick={onClose} disabled={mutation.isPending}>
            {t('common.cancel')}
          </Button>
          <Button
            type="submit"
            form="edit-tenant-form"
            disabled={mutation.isPending || !displayName.trim()}
          >
            {t('common.save')}
          </Button>
        </>
      }
    >
      {tenant.isLoading || !tenant.data ? (
        <LoadingSpinner />
      ) : (
        <form id="edit-tenant-form" className="space-y-4" onSubmit={handleSubmit}>
          <p className="text-sm text-muted-foreground">{t('tenants.readOnlyFields')}</p>
          <Input label={t('tenants.legalName')} value={tenant.data.legalName} disabled readOnly />
          <Input label={t('common.status')} value={tenant.data.status} disabled readOnly />
          <Input
            label={t('tenants.displayName')}
            required
            value={displayName}
            onChange={(e) => {
              setDisplayName(e.target.value);
            }}
          />
          <Input
            label={t('tenants.planId')}
            value={planId}
            onChange={(e) => {
              setPlanId(e.target.value);
            }}
          />
        </form>
      )}
    </Dialog>
  );
}

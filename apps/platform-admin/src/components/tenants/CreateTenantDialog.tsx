import { useMutation, useQueryClient } from '@tanstack/react-query';
import { useState, type SubmitEvent } from 'react';

import { Button } from '@/components/ui/Button';
import { Dialog } from '@/components/ui/Dialog';
import { Input } from '@/components/ui/Input';
import { createTenant } from '@/lib/api/tenants';
import { useI18n } from '@/lib/i18n/context';
import { useToast } from '@/hooks/useToast';

type CreateTenantDialogProps = {
  open: boolean;
  onClose: () => void;
};

const empty = {
  legalName: '',
  displayName: '',
  planId: '',
  adminEmail: '',
  cnpj: '',
};

export function CreateTenantDialog({ open, onClose }: CreateTenantDialogProps) {
  const { t } = useI18n();
  const toast = useToast();
  const queryClient = useQueryClient();
  const [form, setForm] = useState(empty);

  const mutation = useMutation({
    mutationFn: createTenant,
    onSuccess: (result) => {
      toast.success(`Tenant created. Temp password: ${result.adminTemporaryPassword}`);
      void queryClient.invalidateQueries({ queryKey: ['tenants'] });
      setForm(empty);
      onClose();
    },
    onError: () => {
      toast.error(t('common.somethingWentWrong'));
    },
  });

  function handleSubmit(event: SubmitEvent<HTMLFormElement>) {
    event.preventDefault();
    mutation.mutate(form);
  }

  return (
    <Dialog
      open={open}
      title={t('tenants.new')}
      onClose={onClose}
      footer={
        <>
          <Button type="button" variant="secondary" onClick={onClose} disabled={mutation.isPending}>
            {t('common.cancel')}
          </Button>
          <Button type="submit" form="create-tenant-form" disabled={mutation.isPending}>
            {t('common.save')}
          </Button>
        </>
      }
    >
      <form id="create-tenant-form" className="space-y-4" onSubmit={handleSubmit}>
        {(
          [
            ['legalName', t('tenants.legalName')],
            ['displayName', t('tenants.displayName')],
            ['planId', t('tenants.planId')],
            ['adminEmail', t('users.email')],
            ['cnpj', 'CNPJ'],
          ] as const
        ).map(([field, label]) => (
          <Input
            key={field}
            label={label}
            name={field}
            required
            value={form[field]}
            onChange={(e) => {
              setForm((prev) => ({ ...prev, [field]: e.target.value }));
            }}
          />
        ))}
      </form>
    </Dialog>
  );
}

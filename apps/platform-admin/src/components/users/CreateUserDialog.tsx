import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';
import { useState, type SubmitEvent } from 'react';

import { Dialog } from '@/components/ui/Dialog';
import { Button } from '@/components/ui/Button';
import { Input } from '@/components/ui/Input';
import { Select } from '@/components/ui/Select';
import { createPlatformTenantUser } from '@/lib/api/users';
import { fetchTenants } from '@/lib/api/tenants';
import { useI18n } from '@/lib/i18n/context';
import { useToast } from '@/hooks/useToast';

type CreateUserDialogProps = {
  open: boolean;
  onClose: () => void;
};

export function CreateUserDialog({ open, onClose }: CreateUserDialogProps) {
  const { t } = useI18n();
  const toast = useToast();
  const queryClient = useQueryClient();
  const [tenantId, setTenantId] = useState('');
  const [name, setName] = useState('');
  const [email, setEmail] = useState('');
  const [role, setRole] = useState('Admin');

  const tenants = useQuery({
    queryKey: ['tenants', 'create-user'],
    queryFn: () => fetchTenants({ limit: 100 }),
    enabled: open,
  });

  const create = useMutation({
    mutationFn: () =>
      createPlatformTenantUser(tenantId, {
        name: name.trim(),
        email: email.trim(),
        role,
      }),
    onSuccess: (result) => {
      toast.success(`${t('users.created')}: ${result.temporaryPassword}`);
      void queryClient.invalidateQueries({ queryKey: ['platform-users'] });
      setName('');
      setEmail('');
      setRole('Admin');
      setTenantId('');
      onClose();
    },
    onError: () => {
      toast.error(t('common.somethingWentWrong'));
    },
  });

  function handleSubmit(event: SubmitEvent<HTMLFormElement>) {
    event.preventDefault();
    if (!tenantId || !name.trim() || !email.trim()) {
      return;
    }
    create.mutate();
  }

  return (
    <Dialog
      open={open}
      title={t('users.new')}
      onClose={onClose}
      footer={
        <>
          <Button type="button" variant="secondary" onClick={onClose} disabled={create.isPending}>
            {t('common.cancel')}
          </Button>
          <Button
            type="submit"
            form="create-user-form"
            disabled={create.isPending || !tenantId || !name.trim() || !email.trim()}
          >
            {t('common.save')}
          </Button>
        </>
      }
    >
      <form id="create-user-form" className="space-y-4" onSubmit={handleSubmit}>
        <Select
          label={t('users.tenant')}
          value={tenantId}
          onChange={(e) => {
            setTenantId(e.target.value);
          }}
          required
        >
          <option value="">{t('users.selectTenant')}</option>
          {(tenants.data?.data ?? []).map((tenant) => (
            <option key={tenant.id} value={tenant.id}>
              {tenant.displayName}
            </option>
          ))}
        </Select>
        <Input
          label={t('common.name')}
          value={name}
          required
          onChange={(e) => {
            setName(e.target.value);
          }}
        />
        <Input
          label={t('users.email')}
          type="email"
          value={email}
          required
          onChange={(e) => {
            setEmail(e.target.value);
          }}
        />
        <Select
          label={t('users.role')}
          value={role}
          onChange={(e) => {
            setRole(e.target.value);
          }}
        >
          <option value="Admin">Admin</option>
          <option value="Seller">Seller</option>
          <option value="Driver">Driver</option>
        </Select>
      </form>
    </Dialog>
  );
}

import { createFileRoute, useNavigate } from '@tanstack/react-router';
import { useMutation } from '@tanstack/react-query';
import { useState, type SubmitEvent } from 'react';

import { Button } from '@/components/ui/Button';
import { Card } from '@/components/ui/Card';
import { Input } from '@/components/ui/Input';
import { PageHeader } from '@/components/ui/PageHeader';
import { createTenant } from '@/lib/api/tenants';
import { useI18n } from '@/lib/i18n/context';
import { useToast } from '@/hooks/useToast';

export const Route = createFileRoute('/_authenticated/tenants/new')({
  component: NewTenantPage,
});

function NewTenantPage() {
  const { t } = useI18n();
  const navigate = useNavigate();
  const toast = useToast();
  const [form, setForm] = useState({
    legalName: '',
    displayName: '',
    planId: '',
    adminEmail: '',
    cnpj: '',
  });

  const mutation = useMutation({
    mutationFn: createTenant,
    onSuccess: (result) => {
      toast.success(`Tenant created. Temp password: ${result.adminTemporaryPassword}`);
      void navigate({ to: '/tenants/$id', params: { id: result.tenantId } });
    },
  });

  function handleSubmit(event: SubmitEvent<HTMLFormElement>) {
    event.preventDefault();
    mutation.mutate(form);
  }

  return (
    <div className="mx-auto max-w-xl space-y-4">
      <PageHeader title={t('tenants.new')} />
      <Card className="p-4">
        <form className="space-y-4" onSubmit={handleSubmit}>
          {(['legalName', 'displayName', 'planId', 'adminEmail', 'cnpj'] as const).map((field) => (
            <Input
              key={field}
              label={field}
              name={field}
              required
              value={form[field]}
              onChange={(e) => {
                setForm((prev) => ({ ...prev, [field]: e.target.value }));
              }}
            />
          ))}
          <Button type="submit" disabled={mutation.isPending}>
            {t('common.save')}
          </Button>
        </form>
      </Card>
    </div>
  );
}

import { createFileRoute } from '@tanstack/react-router';
import { useMutation } from '@tanstack/react-query';
import { useState, type SubmitEvent } from 'react';

import { Button } from '@/components/ui/Button';
import { Card } from '@/components/ui/Card';
import { Input } from '@/components/ui/Input';
import { PageHeader } from '@/components/ui/PageHeader';
import { Textarea } from '@/components/ui/Textarea';
import { scheduleMaintenance } from '@/lib/api/health';
import { useI18n } from '@/lib/i18n/context';
import { useToast } from '@/hooks/useToast';

export const Route = createFileRoute('/_authenticated/maintenance/')({
  component: MaintenancePage,
});

function MaintenancePage() {
  const { t } = useI18n();
  const toast = useToast();
  const [form, setForm] = useState({
    tenantId: '',
    message: '',
    startsAt: '',
    endsAt: '',
  });

  const mutation = useMutation({
    mutationFn: scheduleMaintenance,
    onSuccess: () => {
      toast.success('Maintenance scheduled');
    },
  });

  function handleSubmit(event: SubmitEvent<HTMLFormElement>) {
    event.preventDefault();
    mutation.mutate({
      tenantId: form.tenantId.trim() || undefined,
      message: form.message,
      startsAt: new Date(form.startsAt).toISOString(),
      endsAt: new Date(form.endsAt).toISOString(),
    });
  }

  return (
    <div className="mx-auto max-w-xl space-y-4">
      <PageHeader title={t('maintenance.title')} />
      <Card className="p-4">
        <form className="space-y-4" onSubmit={handleSubmit}>
          <Input
            label={t('maintenance.tenantOptional')}
            value={form.tenantId}
            onChange={(e) => {
              setForm((prev) => ({ ...prev, tenantId: e.target.value }));
            }}
          />
          <Textarea
            label={t('maintenance.message')}
            required
            value={form.message}
            onChange={(e) => {
              setForm((prev) => ({ ...prev, message: e.target.value }));
            }}
          />
          <Input
            label={t('maintenance.startsAt')}
            type="datetime-local"
            required
            value={form.startsAt}
            onChange={(e) => {
              setForm((prev) => ({ ...prev, startsAt: e.target.value }));
            }}
          />
          <Input
            label={t('maintenance.endsAt')}
            type="datetime-local"
            required
            value={form.endsAt}
            onChange={(e) => {
              setForm((prev) => ({ ...prev, endsAt: e.target.value }));
            }}
          />
          <Button type="submit" disabled={mutation.isPending}>
            {t('maintenance.schedule')}
          </Button>
        </form>
      </Card>
    </div>
  );
}

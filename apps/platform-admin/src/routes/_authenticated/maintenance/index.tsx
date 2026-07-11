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
    <div className="mx-auto w-full max-w-4xl space-y-6">
      <PageHeader title={t('maintenance.title')} description={t('maintenance.description')} />

      <Card className="platform-form-panel overflow-hidden p-0">
        <div className="platform-form-panel-accent" aria-hidden />
        <div className="border-b border-hairline bg-surface-muted/50 px-6 py-4 md:px-10 md:py-5">
          <p className="text-sm leading-relaxed text-muted-foreground">
            {t('maintenance.scopeHint')}
          </p>
        </div>

        <form className="space-y-6 px-6 py-6 md:space-y-8 md:px-10 md:py-8" onSubmit={handleSubmit}>
          <Input
            label={t('maintenance.tenantOptional')}
            value={form.tenantId}
            placeholder="01900001-0000-7000-8000-000000000001"
            className="h-11 text-base md:h-12"
            onChange={(e) => {
              setForm((prev) => ({ ...prev, tenantId: e.target.value }));
            }}
          />

          <Textarea
            label={t('maintenance.message')}
            required
            rows={5}
            value={form.message}
            placeholder="Scheduled maintenance — portal checkout may be unavailable."
            className="min-h-36 resize-y text-base leading-relaxed md:min-h-40"
            onChange={(e) => {
              setForm((prev) => ({ ...prev, message: e.target.value }));
            }}
          />

          <div className="grid gap-6 md:grid-cols-2 md:gap-8">
            <Input
              label={t('maintenance.startsAt')}
              type="datetime-local"
              required
              value={form.startsAt}
              className="h-11 text-base md:h-12"
              onChange={(e) => {
                setForm((prev) => ({ ...prev, startsAt: e.target.value }));
              }}
            />
            <Input
              label={t('maintenance.endsAt')}
              type="datetime-local"
              required
              value={form.endsAt}
              className="h-11 text-base md:h-12"
              onChange={(e) => {
                setForm((prev) => ({ ...prev, endsAt: e.target.value }));
              }}
            />
          </div>

          <div className="flex flex-col gap-3 border-t border-hairline pt-6 md:flex-row md:items-center md:justify-end md:pt-8">
            <Button
              type="submit"
              disabled={mutation.isPending}
              className="h-11 min-h-11 w-full px-8 text-base shadow-md shadow-primary/20 md:w-auto md:min-w-[14rem]"
            >
              {t('maintenance.schedule')}
            </Button>
          </div>
        </form>
      </Card>
    </div>
  );
}

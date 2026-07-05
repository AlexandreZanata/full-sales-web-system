import { useState, type SubmitEvent } from 'react';

import { Button } from '@/components/ui/Button';
import { Input } from '@/components/ui/Input';
import { useToast } from '@/hooks/useToast';
import type { SiteSettings } from '@/lib/api/settings';
import { useI18n } from '@/lib/i18n/context';
import { translateFormError } from '@/lib/i18n/labels';
import { validateSalesContactPhone } from '@/lib/settings/phone';

type SalesContactFormProps = {
  settings: SiteSettings;
  onSave: (salesContactPhone: string) => Promise<SiteSettings>;
  onSaved: () => void;
};

export function SalesContactForm({ settings, onSave, onSaved }: SalesContactFormProps) {
  const { t } = useI18n();
  const toast = useToast();
  const [salesContactPhone, setSalesContactPhone] = useState(settings.salesContactPhone ?? '');
  const [error, setError] = useState<string | undefined>();
  const [submitting, setSubmitting] = useState(false);

  async function handleSubmit(event: SubmitEvent<HTMLFormElement>) {
    event.preventDefault();
    const nextError = validateSalesContactPhone(salesContactPhone);
    setError(nextError);
    if (nextError) {
      return;
    }

    setSubmitting(true);
    try {
      await onSave(salesContactPhone.trim());
      onSaved();
      toast.success(t('settings.toast.contactSaved'));
    } catch {
      toast.error(t('errors.actionFailed'));
    } finally {
      setSubmitting(false);
    }
  }

  return (
    <form className="space-y-4" onSubmit={(event) => void handleSubmit(event)}>
      <Input
        label={t('settings.contact.salesPhone')}
        name="salesContactPhone"
        type="tel"
        autoComplete="tel"
        value={salesContactPhone}
        error={translateFormError(t, error)}
        onChange={(event) => {
          setSalesContactPhone(event.target.value);
        }}
      />
      <Button type="submit" disabled={submitting}>
        {submitting ? t('settings.contact.saving') : t('settings.contact.save')}
      </Button>
    </form>
  );
}

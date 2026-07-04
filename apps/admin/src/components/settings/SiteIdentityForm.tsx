import { useState, type SubmitEvent } from 'react';

import { Button } from '@/components/ui/Button';
import { Input } from '@/components/ui/Input';
import { useToast } from '@/hooks/useToast';
import type { SiteSettings } from '@/lib/api/settings';
import { useI18n } from '@/lib/i18n/context';
import { translateFormError } from '@/lib/i18n/labels';
import { hasSiteIdentityErrors, validateSiteIdentityForm } from '@/lib/settings/validation';

type SiteIdentityFormProps = {
  settings: SiteSettings;
  onSave: (displayName: string) => Promise<SiteSettings>;
  onSaved: () => void;
};

export function SiteIdentityForm({ settings, onSave, onSaved }: SiteIdentityFormProps) {
  const { t } = useI18n();
  const toast = useToast();
  const [displayName, setDisplayName] = useState(settings.displayName);
  const [errors, setErrors] = useState<ReturnType<typeof validateSiteIdentityForm>>({});
  const [submitting, setSubmitting] = useState(false);

  async function handleSubmit(event: SubmitEvent<HTMLFormElement>) {
    event.preventDefault();
    const nextErrors = validateSiteIdentityForm({ displayName });
    setErrors(nextErrors);
    if (hasSiteIdentityErrors(nextErrors)) {
      return;
    }

    setSubmitting(true);
    try {
      await onSave(displayName.trim());
      onSaved();
      toast.success(t('settings.toast.identitySaved'));
    } catch {
      toast.error(t('errors.actionFailed'));
    } finally {
      setSubmitting(false);
    }
  }

  return (
    <form className="space-y-4" onSubmit={(event) => void handleSubmit(event)}>
      <Input
        label={t('settings.identity.displayName')}
        name="displayName"
        value={displayName}
        error={translateFormError(t, errors.displayName)}
        onChange={(event) => {
          setDisplayName(event.target.value);
        }}
      />
      <Button type="submit" disabled={submitting}>
        {submitting ? t('settings.identity.saving') : t('settings.identity.save')}
      </Button>
    </form>
  );
}

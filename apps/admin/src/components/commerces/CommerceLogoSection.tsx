import { useState, type SubmitEvent } from 'react';

import { FileUploadField } from '@/components/uploads/FileUploadField';
import { useToast } from '@/hooks/useToast';
import { updateCommerceLogo } from '@/lib/api/commerces';
import { useI18n } from '@/lib/i18n/context';

type CommerceLogoSectionProps = {
  commerceId: string;
};

export function CommerceLogoSection({ commerceId }: CommerceLogoSectionProps) {
  const { t } = useI18n();
  const toast = useToast();
  const [logoFileId, setLogoFileId] = useState('');
  const [saving, setSaving] = useState(false);

  async function handleLogoChange(fileId: string) {
    setLogoFileId(fileId);
    setSaving(true);
    try {
      await updateCommerceLogo(commerceId, fileId);
      toast.success(t('commerces.toast.logoUpdated'));
    } catch {
      toast.error(t('errors.actionFailed'));
      setLogoFileId('');
    } finally {
      setSaving(false);
    }
  }

  return (
    <form
      className="space-y-2"
      onSubmit={(event: SubmitEvent<HTMLFormElement>) => {
        event.preventDefault();
      }}
    >
      <FileUploadField
        label={t('commerces.logo.label')}
        fileId={logoFileId}
        onChange={(fileId) => void handleLogoChange(fileId)}
        entityType="Commerce"
        entityId={commerceId}
      />
      {saving ? (
        <p className="text-xs text-muted-foreground">{t('commerces.logo.saving')}</p>
      ) : null}
    </form>
  );
}

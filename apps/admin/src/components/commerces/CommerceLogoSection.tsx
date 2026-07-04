import { useState } from 'react';

import { FileUploadField } from '@/components/uploads/FileUploadField';
import { useToast } from '@/hooks/useToast';
import { updateCommerceLogo } from '@/lib/api/commerces';
import { useI18n } from '@/lib/i18n/context';

type CommerceLogoSectionProps = {
  commerceId: string;
  logoFileId?: string;
};

export function CommerceLogoSection({ commerceId, logoFileId }: CommerceLogoSectionProps) {
  const { t } = useI18n();
  const toast = useToast();
  const [currentLogoFileId, setCurrentLogoFileId] = useState(logoFileId ?? '');
  const [saving, setSaving] = useState(false);

  async function handleLogoChange(fileId: string) {
    setCurrentLogoFileId(fileId);
    setSaving(true);
    try {
      await updateCommerceLogo(commerceId, fileId);
      toast.success(t('commerces.toast.logoUpdated'));
    } catch {
      toast.error(t('errors.actionFailed'));
      setCurrentLogoFileId(logoFileId ?? '');
    } finally {
      setSaving(false);
    }
  }

  return (
    <form
      className="space-y-2"
      onSubmit={(event) => {
        event.preventDefault();
      }}
    >
      <FileUploadField
        label={t('commerces.logo.label')}
        fileId={currentLogoFileId}
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

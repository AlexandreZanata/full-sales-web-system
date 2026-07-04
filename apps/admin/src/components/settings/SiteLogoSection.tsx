import { useQueryClient } from '@tanstack/react-query';
import { useState } from 'react';

import { FileUploadField } from '@/components/uploads/FileUploadField';
import { useToast } from '@/hooks/useToast';
import { updateSiteLogo } from '@/lib/api/settings';
import { resolveTenantIdFromSession } from '@/lib/auth/tenantId';
import { useI18n } from '@/lib/i18n/context';
import { siteSettingsQueryKey } from '@/lib/settings/useSiteSettings';

type SiteLogoSectionProps = {
  logoFileId?: string;
};

export function SiteLogoSection({ logoFileId: initialLogoFileId }: SiteLogoSectionProps) {
  const { t } = useI18n();
  const toast = useToast();
  const queryClient = useQueryClient();
  const tenantId = resolveTenantIdFromSession();
  const [logoFileId, setLogoFileId] = useState(initialLogoFileId ?? '');
  const [saving, setSaving] = useState(false);

  async function handleLogoChange(fileId: string) {
    setLogoFileId(fileId);
    setSaving(true);
    try {
      await updateSiteLogo(fileId);
      await queryClient.invalidateQueries({ queryKey: siteSettingsQueryKey() });
      toast.success(t('settings.toast.logoSaved'));
    } catch {
      toast.error(t('errors.actionFailed'));
      setLogoFileId(initialLogoFileId ?? '');
    } finally {
      setSaving(false);
    }
  }

  return (
    <div className="space-y-2">
      <FileUploadField
        label={t('settings.logo.label')}
        fileId={logoFileId}
        onChange={(fileId) => void handleLogoChange(fileId)}
        entityType="Tenant"
        entityId={tenantId}
      />
      {saving ? <p className="text-xs text-muted-foreground">{t('settings.logo.saving')}</p> : null}
    </div>
  );
}

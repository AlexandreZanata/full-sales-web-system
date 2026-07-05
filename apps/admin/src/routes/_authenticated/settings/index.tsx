import { createFileRoute } from '@tanstack/react-router';
import { useQueryClient } from '@tanstack/react-query';

import { SiteIdentityForm } from '@/components/settings/SiteIdentityForm';
import { SalesContactForm } from '@/components/settings/SalesContactForm';
import { SiteLogoSection } from '@/components/settings/SiteLogoSection';
import { Card } from '@/components/ui/Card';
import { LoadingSpinner } from '@/components/ui/LoadingSpinner';
import { PageHeader } from '@/components/ui/PageHeader';
import { updateSalesContactPhone, updateSettings } from '@/lib/api/settings';
import { useI18n } from '@/lib/i18n/context';
import { siteSettingsQueryKey, useSiteSettings } from '@/lib/settings/useSiteSettings';

export const Route = createFileRoute('/_authenticated/settings/')({
  component: SettingsPage,
});

function SettingsPage() {
  const { t } = useI18n();
  const queryClient = useQueryClient();
  const settings = useSiteSettings();

  if (settings.isLoading) {
    return (
      <div className="flex justify-center py-16">
        <LoadingSpinner />
      </div>
    );
  }

  if (settings.isError || !settings.data) {
    return <PageHeader title={t('settings.title')} description={t('settings.loadError')} />;
  }

  const data = settings.data;

  return (
    <div className="space-y-4">
      <PageHeader title={t('settings.title')} description={t('settings.description')} />

      <Card className="space-y-4">
        <p className="text-xs font-semibold uppercase tracking-[0.12em] text-muted-foreground">
          {t('settings.identity.title')}
        </p>
        <SiteIdentityForm
          settings={data}
          onSave={(displayName) => updateSettings({ displayName })}
          onSaved={() => {
            void queryClient.invalidateQueries({ queryKey: siteSettingsQueryKey() });
          }}
        />
      </Card>

      <Card className="space-y-4">
        <p className="text-xs font-semibold uppercase tracking-[0.12em] text-muted-foreground">
          {t('settings.contact.title')}
        </p>
        <p className="text-sm text-muted-foreground">{t('settings.contact.description')}</p>
        <SalesContactForm
          settings={data}
          onSave={(salesContactPhone) => updateSalesContactPhone(salesContactPhone)}
          onSaved={() => {
            void queryClient.invalidateQueries({ queryKey: siteSettingsQueryKey() });
          }}
        />
      </Card>

      <Card className="space-y-4">
        <p className="text-xs font-semibold uppercase tracking-[0.12em] text-muted-foreground">
          {t('settings.logo.title')}
        </p>
        <p className="text-sm text-muted-foreground">{t('settings.logo.description')}</p>
        <SiteLogoSection logoFileId={data.logoFileId} />
      </Card>
    </div>
  );
}

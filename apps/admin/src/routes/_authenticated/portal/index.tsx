import { createFileRoute } from '@tanstack/react-router';

import { PortalBannersSection } from '@/components/portal/PortalBannersSection';
import { PageHeader } from '@/components/ui/PageHeader';
import { useI18n } from '@/lib/i18n/context';

export const Route = createFileRoute('/_authenticated/portal/')({
  component: PortalContentPage,
});

function PortalContentPage() {
  const { t } = useI18n();

  return (
    <div className="space-y-6">
      <PageHeader title={t('portal.title')} description={t('portal.description')} />
      <PortalBannersSection />
    </div>
  );
}

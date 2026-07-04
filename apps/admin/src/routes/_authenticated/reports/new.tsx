import { createFileRoute, useNavigate } from '@tanstack/react-router';

import { CreateReportForm } from '@/components/reports/CreateReportForm';
import { PageBackLink } from '@/components/ui/PageBackLink';
import { PageHeader } from '@/components/ui/PageHeader';
import { generateReport } from '@/lib/api/reports';
import { useI18n } from '@/lib/i18n/context';

export const Route = createFileRoute('/_authenticated/reports/new')({
  component: NewReportPage,
});

function NewReportPage() {
  const navigate = useNavigate();
  const { t } = useI18n();

  return (
    <div>
      <PageHeader
        title={t('reports.generate.title')}
        description={t('reports.generate.description')}
        back={<PageBackLink label={t('common.backTo.reports')} to="/reports" />}
      />

      <CreateReportForm
        onSubmit={generateReport}
        onSuccess={(report) => {
          void navigate({ to: '/reports/$id', params: { id: report.id } });
        }}
      />
    </div>
  );
}

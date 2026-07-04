import { createFileRoute, useNavigate } from '@tanstack/react-router';

import { CreateReportForm } from '@/components/reports/CreateReportForm';
import { PageBackLink } from '@/components/ui/PageBackLink';
import { PageHeader } from '@/components/ui/PageHeader';
import { generateReport } from '@/lib/api/reports';

export const Route = createFileRoute('/_authenticated/reports/new')({
  component: NewReportPage,
});

function NewReportPage() {
  const navigate = useNavigate();

  return (
    <div>
      <PageHeader
        title="Generate report"
        description="Create a signed settlement report for a period and scope."
        back={<PageBackLink label="Back to reports" to="/reports" />}
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

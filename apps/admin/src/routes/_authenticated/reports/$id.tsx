import { createFileRoute } from '@tanstack/react-router';
import { useQuery } from '@tanstack/react-query';
import { useState, type ReactNode } from 'react';

import { ReportTypeBadge } from '@/components/reports/ReportTypeBadge';
import { Button } from '@/components/ui/Button';
import { Card } from '@/components/ui/Card';
import { JsonBlock } from '@/components/ui/JsonBlock';
import { LoadingSpinner } from '@/components/ui/LoadingSpinner';
import { PageBackLink } from '@/components/ui/PageBackLink';
import { PageHeader } from '@/components/ui/PageHeader';
import { useToast } from '@/hooks/useToast';
import { fetchReport, verifyReport } from '@/lib/api/reports';
import { formatDateTime } from '@/lib/formatDateTime';
import { formatReportPeriod } from '@/lib/reports/formatPeriod';
import { buildReportVerifyUrl } from '@/lib/reports/verifyUrl';

export const Route = createFileRoute('/_authenticated/reports/$id')({
  component: ReportDetailPage,
});

function ReportDetailPage() {
  const { id } = Route.useParams();
  const toast = useToast();
  const [copying, setCopying] = useState(false);

  const report = useQuery({
    queryKey: ['reports', id],
    queryFn: () => fetchReport(id),
  });

  const verify = useQuery({
    queryKey: ['reports', id, 'verify'],
    queryFn: () => verifyReport(id),
    enabled: Boolean(report.data),
  });

  let payload: unknown = null;
  if (report.data?.canonicalPayload) {
    try {
      payload = JSON.parse(report.data.canonicalPayload) as unknown;
    } catch {
      payload = report.data.canonicalPayload;
    }
  }

  async function copyVerifyUrl() {
    setCopying(true);
    try {
      await navigator.clipboard.writeText(buildReportVerifyUrl(id));
      toast.success('Verify URL copied');
    } catch {
      toast.error('Unable to copy verify URL');
    } finally {
      setCopying(false);
    }
  }

  if (report.isLoading) {
    return (
      <div className="flex justify-center py-16">
        <LoadingSpinner />
      </div>
    );
  }

  if (!report.data) {
    return (
      <PageHeader
        title="Report not found"
        back={<PageBackLink label="Back to reports" to="/reports" />}
      />
    );
  }

  const detail = report.data;

  return (
    <div className="space-y-6">
      <PageHeader
        title={`Report ${detail.id.slice(0, 8)}…`}
        description={formatReportPeriod(detail.periodStart, detail.periodEnd)}
        back={<PageBackLink label="Back to reports" to="/reports" />}
        actions={
          <div className="flex flex-wrap gap-2">
            <Button variant="secondary" disabled={copying} onClick={() => void copyVerifyUrl()}>
              Copy verify URL
            </Button>
            <Button
              variant="secondary"
              onClick={() => {
                window.open(buildReportVerifyUrl(id), '_blank', 'noopener,noreferrer');
              }}
            >
              Open verify endpoint
            </Button>
          </div>
        }
      />

      <Card className="space-y-3 p-5">
        <DetailRow label="Type" value={<ReportTypeBadge reportType={detail.reportType} />} />
        <DetailRow
          label="Period"
          value={formatReportPeriod(detail.periodStart, detail.periodEnd)}
        />
        <DetailRow label="Generated" value={formatDateTime(detail.generatedAt)} />
        <DetailRow
          label="Public key"
          value={<span className="font-mono text-xs">{detail.publicKeyId}</span>}
        />
        <DetailRow
          label="Signature"
          value={
            <span className="break-all font-mono text-xs text-muted-foreground">
              {detail.signature}
            </span>
          }
        />
        {verify.data ? (
          <DetailRow
            label="Verify result"
            value={
              <span className={verify.data.valid ? 'text-emerald-700' : 'text-red-700'}>
                {verify.data.valid ? 'Valid signature' : 'Invalid signature'}
              </span>
            }
          />
        ) : verify.isLoading ? (
          <DetailRow label="Verify result" value="Checking…" />
        ) : null}
      </Card>

      <div>
        <h2 className="mb-3 text-base font-semibold text-foreground">Canonical payload</h2>
        <JsonBlock value={payload} defaultOpen />
      </div>
    </div>
  );
}

function DetailRow({ label, value }: { label: string; value: ReactNode }) {
  return (
    <div className="flex flex-col gap-1 sm:flex-row sm:items-start sm:justify-between">
      <span className="text-xs font-semibold uppercase tracking-[0.12em] text-muted-foreground">
        {label}
      </span>
      <span className="max-w-full text-sm text-foreground sm:text-right">{value}</span>
    </div>
  );
}

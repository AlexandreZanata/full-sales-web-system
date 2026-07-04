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
import {
  downloadReportExport,
  fetchReport,
  verifyReport,
  type ReportExportFormat,
} from '@/lib/api/reports';
import { formatDateTime } from '@/lib/formatDateTime';
import { useI18n } from '@/lib/i18n/context';
import { formatReportPeriod } from '@/lib/reports/formatPeriod';
import { buildReportVerifyUrl } from '@/lib/reports/verifyUrl';

const EXPORT_FORMATS: ReportExportFormat[] = ['pdf', 'csv', 'xlsx'];

export const Route = createFileRoute('/_authenticated/reports/$id')({
  component: ReportDetailPage,
});

function ReportDetailPage() {
  const { id } = Route.useParams();
  const { t } = useI18n();
  const toast = useToast();
  const [copying, setCopying] = useState(false);
  const [exportingFormat, setExportingFormat] = useState<ReportExportFormat | null>(null);

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
      toast.success(t('reports.toast.verifyUrlCopied'));
    } catch {
      toast.error(t('errors.actionFailed'));
    } finally {
      setCopying(false);
    }
  }

  async function handleExport(format: ReportExportFormat) {
    setExportingFormat(format);
    try {
      const { blob, filename } = await downloadReportExport(id, format);
      const url = URL.createObjectURL(blob);
      const anchor = document.createElement('a');
      anchor.href = url;
      anchor.download = filename;
      anchor.click();
      URL.revokeObjectURL(url);
      toast.success(t('reports.toast.exported'));
    } catch {
      toast.error(t('errors.actionFailed'));
    } finally {
      setExportingFormat(null);
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
        title={t('reports.detail.notFound')}
        back={<PageBackLink label={t('common.backTo.reports')} to="/reports" />}
      />
    );
  }

  const detail = report.data;

  return (
    <div className="space-y-6">
      <PageHeader
        title={`${detail.id.slice(0, 8)}…`}
        description={formatReportPeriod(detail.periodStart, detail.periodEnd)}
        back={<PageBackLink label={t('common.backTo.reports')} to="/reports" />}
        actions={
          <div className="flex flex-wrap gap-2">
            <div className="flex flex-wrap items-center gap-2">
              <span className="text-xs font-semibold uppercase tracking-[0.12em] text-muted-foreground">
                {t('reports.detail.download')}
              </span>
              {EXPORT_FORMATS.map((format) => (
                <Button
                  key={format}
                  variant="secondary"
                  disabled={exportingFormat !== null}
                  onClick={() => void handleExport(format)}
                >
                  {exportingFormat === format
                    ? t('reports.export.exporting')
                    : t(`reports.export.${format}`)}
                </Button>
              ))}
            </div>
            <Button variant="secondary" disabled={copying} onClick={() => void copyVerifyUrl()}>
              {t('reports.detail.copyVerifyUrl')}
            </Button>
            <Button
              variant="secondary"
              onClick={() => {
                window.open(buildReportVerifyUrl(id), '_blank', 'noopener,noreferrer');
              }}
            >
              {t('reports.detail.openVerifyEndpoint')}
            </Button>
          </div>
        }
      />

      <Card className="space-y-3 p-5">
        <DetailRow
          label={t('forms.fields.type')}
          value={<ReportTypeBadge reportType={detail.reportType} />}
        />
        <DetailRow
          label={t('forms.fields.period')}
          value={formatReportPeriod(detail.periodStart, detail.periodEnd)}
        />
        <DetailRow label={t('forms.fields.generated')} value={formatDateTime(detail.generatedAt)} />
        <DetailRow
          label={t('forms.fields.publicKey')}
          value={<span className="font-mono text-xs">{detail.publicKeyId}</span>}
        />
        <DetailRow
          label={t('forms.fields.signature')}
          value={
            <span className="break-all font-mono text-xs text-muted-foreground">
              {detail.signature}
            </span>
          }
        />
        {verify.data ? (
          <DetailRow
            label={t('forms.fields.verifyResult')}
            value={
              <span className={verify.data.valid ? 'text-emerald-700' : 'text-red-700'}>
                {verify.data.valid
                  ? t('reports.detail.validSignature')
                  : t('reports.detail.invalidSignature')}
              </span>
            }
          />
        ) : verify.isLoading ? (
          <DetailRow label={t('forms.fields.verifyResult')} value={t('reports.detail.checking')} />
        ) : null}
      </Card>

      <div>
        <h2 className="mb-3 text-base font-semibold text-foreground">
          {t('reports.detail.canonicalPayload')}
        </h2>
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

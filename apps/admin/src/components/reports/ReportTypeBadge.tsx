import type { ReportType } from '@/lib/api/types';
import { useI18n } from '@/lib/i18n/context';
import { translateReportType } from '@/lib/i18n/labels';
import { cn } from '@/lib/utils';

type ReportTypeBadgeProps = {
  reportType: ReportType;
  className?: string;
};

export function ReportTypeBadge({ reportType, className }: ReportTypeBadgeProps) {
  const { t } = useI18n();

  return (
    <span
      className={cn(
        'inline-flex items-center gap-1.5 rounded-full border border-hairline bg-surface-muted px-2 py-0.5 text-xs font-medium text-foreground',
        className,
      )}
    >
      <span className="h-1.5 w-1.5 rounded-full bg-foreground" aria-hidden />
      {translateReportType(t, reportType)}
    </span>
  );
}

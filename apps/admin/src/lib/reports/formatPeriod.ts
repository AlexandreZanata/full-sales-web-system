import { formatDateTime } from '@/lib/formatDateTime';

export function formatReportPeriod(start: string, end: string): string {
  return `${formatDateTime(start)} — ${formatDateTime(end)}`;
}

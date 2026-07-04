import type { ReportType } from '@/lib/api/types';

export const REPORT_TYPES: ReportType[] = ['DailyDriver', 'CommercePeriod', 'Consolidated'];

export const REPORT_TYPE_LABELS: Record<ReportType, string> = {
  DailyDriver: 'Daily driver',
  CommercePeriod: 'Commerce period',
  Consolidated: 'Consolidated',
};

import { getApiBaseUrl } from '@/lib/api/client';

/** Public verify URL for a signed report (ADR-007). */
export function buildReportVerifyUrl(reportId: string): string {
  const origin = typeof window !== 'undefined' ? window.location.origin : '';
  const apiBase = getApiBaseUrl();
  return `${origin}${apiBase}/reports/${reportId}/verify`;
}

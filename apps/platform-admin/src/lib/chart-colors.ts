/** Theme-aligned chart palette (matches platform-theme.css status tokens). */
export const chartColors = {
  active: '#2d9a5f',
  trial: '#4a7fd4',
  pastDue: '#d4920a',
  suspended: '#c97a2e',
  primary: '#d97706',
  muted: '#94a3b8',
  grid: '#e8eaed',
  fraud: {
    low: '#4a7fd4',
    medium: '#d4920a',
    high: '#dc4a4a',
    critical: '#991b1b',
  },
} as const;

export function fraudSeverityColor(severity: string): string {
  const key = severity.toLowerCase();
  if (key === 'critical') return chartColors.fraud.critical;
  if (key === 'high') return chartColors.fraud.high;
  if (key === 'medium') return chartColors.fraud.medium;
  return chartColors.fraud.low;
}

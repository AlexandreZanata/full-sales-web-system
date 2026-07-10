export const platformTokens = {
  shellHeaderBar:
    'flex h-[6.25rem] items-center gap-3 border-b border-hairline bg-surface px-4 md:px-6',
  shellSidebar: 'flex h-full flex-col border-r border-hairline bg-surface-muted',
  shellNavLink:
    'flex items-center gap-2 rounded-md px-3 py-2 text-sm font-medium text-muted-foreground transition hover:bg-surface hover:text-foreground',
  shellNavLinkActive: 'bg-surface text-foreground shadow-sm',
  pageTitle: 'text-2xl font-semibold tracking-tight text-foreground',
} as const;

export type TenantStatus =
  'Active' | 'Trial' | 'PastDue' | 'Suspended' | 'Offboarding' | 'Offboarded';

export function tenantStatusTone(status: string): string {
  switch (status) {
    case 'Active':
      return 'text-status-active';
    case 'Trial':
      return 'text-status-info';
    case 'PastDue':
      return 'text-status-warning';
    case 'Suspended':
      return 'text-status-out-of-stock';
    default:
      return 'text-status-neutral';
  }
}

export function probeStatusTone(status: string): string {
  if (status === 'healthy' || status === 'ok') {
    return 'bg-status-active';
  }
  if (status === 'degraded' || status === 'warning') {
    return 'bg-status-warning';
  }
  return 'bg-status-out-of-stock';
}

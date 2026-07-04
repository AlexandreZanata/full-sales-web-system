export type StatusToken = { dot: string; badge: string };

function token(dot: string, badge: string): StatusToken {
  return { dot, badge };
}

const activeToken = token('bg-status-active', 'text-status-active border-status-active/30');
const warningToken = token('bg-status-warning', 'text-status-warning border-status-warning/30');
const destructiveToken = token('bg-destructive', 'text-destructive border-destructive/30');
const neutralToken = token('bg-status-neutral', 'text-muted-foreground border-hairline');

export type SaleStatus = 'Pending' | 'Confirmed' | 'Cancelled';

const saleStatusColors: Record<SaleStatus, StatusToken> = {
  Pending: warningToken,
  Confirmed: activeToken,
  Cancelled: destructiveToken,
};

export function getSaleStatusToken(status: string): StatusToken {
  if (status in saleStatusColors) {
    return saleStatusColors[status as SaleStatus];
  }
  return neutralToken;
}

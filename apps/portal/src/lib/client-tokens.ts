export type StatusToken = { dot: string; badge: string };

function token(dot: string, badge: string): StatusToken {
  return { dot, badge };
}

const activeToken = token('bg-status-active', 'text-status-active border-status-active/30');
const warningToken = token('bg-status-warning', 'text-status-warning border-status-warning/30');
const infoToken = token('bg-status-info', 'text-status-info border-status-info/30');
const neutralToken = token('bg-status-neutral', 'text-muted-foreground border-hairline');
const destructiveToken = token('bg-destructive', 'text-destructive border-destructive/30');

export type OrderStatus =
  | 'Draft'
  | 'PendingApproval'
  | 'Approved'
  | 'Rejected'
  | 'Picking'
  | 'InTransit'
  | 'Delivered'
  | 'PartiallyDelivered'
  | 'Cancelled';

const orderStatusColors: Record<OrderStatus, StatusToken> = {
  Draft: neutralToken,
  PendingApproval: warningToken,
  Approved: infoToken,
  Rejected: destructiveToken,
  Picking: infoToken,
  InTransit: warningToken,
  Delivered: activeToken,
  PartiallyDelivered: warningToken,
  Cancelled: destructiveToken,
};

export function getOrderStatusToken(status: string): StatusToken {
  if (status in orderStatusColors) {
    return orderStatusColors[status as OrderStatus];
  }
  return neutralToken;
}

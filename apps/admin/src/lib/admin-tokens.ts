export type StatusToken = { dot: string; badge: string; label: string };

function token(dot: string, badge: string, label: string): StatusToken {
  return { dot, badge, label };
}

const activeToken = token('bg-status-active', 'text-status-active border-status-active/30', '');
const warningToken = token('bg-status-warning', 'text-status-warning border-status-warning/30', '');
const infoToken = token('bg-status-info', 'text-status-info border-status-info/30', '');
const neutralToken = token('bg-status-neutral', 'text-muted-foreground border-hairline', '');
const destructiveToken = token('bg-destructive', 'text-destructive border-destructive/30', '');

export const adminTokens = {
  pageTitle: 'text-2xl font-semibold tracking-tight text-foreground',
  sectionTitle: 'text-base font-semibold text-foreground',
  label: 'text-xs font-semibold uppercase tracking-[0.12em] text-muted-foreground',
  hairlineBorder: 'border border-hairline',
  surface: 'bg-surface',
  sidebarWidth: 'w-60',
  shellTopBarHeight: 'h-[5.75rem]',
  shellBrandBar:
    'flex shrink-0 items-center gap-3 border-b border-hairline bg-surface px-5 h-[6.25rem]',
  shellHeaderBar:
    'flex shrink-0 items-center gap-3 border-b border-hairline bg-surface px-4 h-[6.25rem] md:justify-end md:px-6',
  sidebarActive: 'bg-foreground text-background hover:bg-foreground/90 [&_svg]:text-background',
  sidebarItem:
    'flex items-center gap-2 rounded-md px-3 py-2 text-sm font-medium text-foreground hover:bg-surface-muted transition-colors min-h-10',
} as const;

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

export type SaleStatus = 'Pending' | 'Confirmed' | 'Cancelled';

export type DeliveryStatus = 'Waiting' | 'InTransit' | 'Delivered' | 'Failed';

export const orderStatusColors: Record<OrderStatus, StatusToken> = {
  Draft: { ...neutralToken, label: 'Draft' },
  PendingApproval: { ...warningToken, label: 'Pending approval' },
  Approved: { ...infoToken, label: 'Approved' },
  Rejected: { ...destructiveToken, label: 'Rejected' },
  Picking: { ...infoToken, label: 'Picking' },
  InTransit: { ...warningToken, label: 'In transit' },
  Delivered: { ...activeToken, label: 'Delivered' },
  PartiallyDelivered: { ...warningToken, label: 'Partially delivered' },
  Cancelled: { ...destructiveToken, label: 'Cancelled' },
};

export const saleStatusColors: Record<SaleStatus, StatusToken> = {
  Pending: { ...warningToken, label: 'Pending' },
  Confirmed: { ...activeToken, label: 'Confirmed' },
  Cancelled: { ...destructiveToken, label: 'Cancelled' },
};

export const deliveryStatusColors: Record<DeliveryStatus, StatusToken> = {
  Waiting: { ...neutralToken, label: 'Waiting' },
  InTransit: { ...infoToken, label: 'In transit' },
  Delivered: { ...activeToken, label: 'Delivered' },
  Failed: { ...destructiveToken, label: 'Failed' },
};

export function getOrderStatusToken(status: OrderStatus): StatusToken {
  return orderStatusColors[status];
}

export function getSaleStatusToken(status: SaleStatus): StatusToken {
  return saleStatusColors[status];
}

export function getDeliveryStatusToken(status: DeliveryStatus): StatusToken {
  return deliveryStatusColors[status];
}

import type { OrderStatus } from '@/lib/admin-tokens';

export const ORDER_STATUSES: OrderStatus[] = [
  'Draft',
  'PendingApproval',
  'Approved',
  'Rejected',
  'Picking',
  'InTransit',
  'Delivered',
  'PartiallyDelivered',
  'Cancelled',
];

export type OrderStatusFilter = OrderStatus | '';

export const ORDER_STATUS_FILTER_LABELS: Record<OrderStatusFilter, string> = {
  '': 'All statuses',
  Draft: 'Draft',
  PendingApproval: 'Pending approval',
  Approved: 'Approved',
  Rejected: 'Rejected',
  Picking: 'Picking',
  InTransit: 'In transit',
  Delivered: 'Delivered',
  PartiallyDelivered: 'Partially delivered',
  Cancelled: 'Cancelled',
};

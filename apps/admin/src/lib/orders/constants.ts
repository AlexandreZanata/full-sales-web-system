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

export const ORDER_STATUS_FILTERS: OrderStatusFilter[] = ['', ...ORDER_STATUSES];

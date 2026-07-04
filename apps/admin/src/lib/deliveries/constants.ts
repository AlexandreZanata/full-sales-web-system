import type { DeliveryStatus } from '@/lib/admin-tokens';

export const DELIVERY_STATUSES: DeliveryStatus[] = ['Waiting', 'InTransit', 'Delivered', 'Failed'];

export type DeliveryStatusFilter = DeliveryStatus | '';

export const DELIVERY_STATUS_FILTER_LABELS: Record<DeliveryStatusFilter, string> = {
  '': 'All statuses',
  Waiting: 'Waiting',
  InTransit: 'In transit',
  Delivered: 'Delivered',
  Failed: 'Failed',
};

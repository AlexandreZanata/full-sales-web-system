import type { OrderStatus } from '@/lib/api/types';

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

export const ORDER_TIMELINE: OrderStatus[] = [
  'Draft',
  'PendingApproval',
  'Approved',
  'Picking',
  'InTransit',
  'Delivered',
];

export type DeliveryAddressOption = {
  id: string;
  label: string;
  commerceId: string;
  type: 'Delivery' | 'Billing';
};

/** Dev seed addresses — until GET /v1/portal/addresses exists (GAP-039-03). */
export const SEED_DELIVERY_ADDRESSES: DeliveryAddressOption[] = [
  {
    id: '01900001-0011-7000-8000-000000000002',
    commerceId: '01900001-0010-7000-8000-000000000001',
    type: 'Delivery',
    label: 'Rua Augusta, 200 — Centro, São Paulo/SP',
  },
  {
    id: '01900001-0011-7000-8000-000000000001',
    commerceId: '01900001-0010-7000-8000-000000000001',
    type: 'Billing',
    label: 'Av. Paulista, 1000 — Centro, São Paulo/SP',
  },
];

export function addressesForCommerce(commerceId?: string): DeliveryAddressOption[] {
  if (!commerceId) {
    return SEED_DELIVERY_ADDRESSES.filter((address) => address.type === 'Delivery');
  }
  return SEED_DELIVERY_ADDRESSES.filter(
    (address) => address.commerceId === commerceId && address.type === 'Delivery',
  );
}

export function orderStatusIndex(status: string): number {
  const index = ORDER_TIMELINE.indexOf(status as OrderStatus);
  return index >= 0 ? index : -1;
}

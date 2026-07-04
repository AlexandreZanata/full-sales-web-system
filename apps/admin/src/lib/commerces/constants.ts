export const ADDRESS_TYPES = ['Billing', 'Delivery'] as const;

export type AddressTypeOption = (typeof ADDRESS_TYPES)[number];

export const ADDRESS_TYPE_LABELS: Record<AddressTypeOption, string> = {
  Billing: 'Billing',
  Delivery: 'Delivery',
};

export type ActiveFilter = '' | 'true' | 'false';

export const ACTIVE_FILTER_LABELS: Record<ActiveFilter, string> = {
  '': 'All statuses',
  true: 'Active only',
  false: 'Inactive only',
};

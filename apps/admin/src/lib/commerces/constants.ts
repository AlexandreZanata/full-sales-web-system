export const ADDRESS_TYPES = ['Billing', 'Delivery'] as const;

export type AddressTypeOption = (typeof ADDRESS_TYPES)[number];

export type ActiveFilter = '' | 'true' | 'false';

export const ACTIVE_FILTERS: ActiveFilter[] = ['', 'true', 'false'];

export const USER_ROLES = ['Admin', 'Driver', 'Seller', 'CommerceContact'] as const;

export type UserRoleOption = (typeof USER_ROLES)[number];

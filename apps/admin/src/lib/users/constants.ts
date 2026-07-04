export const USER_ROLES = ['Admin', 'Driver', 'Seller', 'CommerceContact'] as const;

export type UserRoleOption = (typeof USER_ROLES)[number];

export const USER_ROLE_LABELS: Record<UserRoleOption, string> = {
  Admin: 'Admin',
  Driver: 'Driver',
  Seller: 'Seller',
  CommerceContact: 'Commerce contact',
};

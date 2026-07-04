import { describe, expect, it } from 'vitest';

import { adminNavItems } from '@/lib/admin-nav';

/** Contract: Phase 27 sidebar table — 10 nav items, unique routes. */
describe('adminNavItems — Phase 27 sidebar contract', () => {
  it('lists all 10 admin panel routes', () => {
    expect(adminNavItems).toHaveLength(10);
    expect(adminNavItems.map((item) => item.to)).toEqual([
      '/',
      '/users',
      '/commerces',
      '/products',
      '/inventory',
      '/orders',
      '/deliveries',
      '/sales',
      '/reports',
      '/audit',
    ]);
  });
});

import { describe, expect, it } from 'vitest';

import { adminNavItems } from '@/lib/admin-nav';

/** Contract: Phase 27 sidebar table — nav items, unique routes. */
describe('adminNavItems — Phase 27 sidebar contract', () => {
  it('lists all 13 admin panel routes', () => {
    expect(adminNavItems).toHaveLength(13);
    expect(adminNavItems.map((item) => item.to)).toEqual([
      '/',
      '/users',
      '/commerces',
      '/products',
      '/categories',
      '/inventory',
      '/orders',
      '/deliveries',
      '/sales',
      '/reports',
      '/audit',
      '/settings',
      '/portal',
    ]);
  });
});

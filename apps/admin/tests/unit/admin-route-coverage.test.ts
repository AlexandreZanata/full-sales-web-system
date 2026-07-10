/**
 * Contract: Phase 36 — every sidebar route is registered in TanStack Router.
 */
import { readFileSync } from 'node:fs';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

import { describe, expect, it } from 'vitest';

import { adminNavItems } from '@/lib/admin-nav';

const testDir = dirname(fileURLToPath(import.meta.url));
const routeTreeSource = readFileSync(resolve(testDir, '../../src/routeTree.gen.ts'), 'utf8');

describe('admin route coverage — Phase 36 contract', () => {
  it('registers_every_sidebar_route_in_route_tree', () => {
    for (const item of adminNavItems) {
      const normalized = item.to === '/' ? "'/'" : `'${item.to}'`;
      expect(routeTreeSource).toContain(normalized);
    }
  });

  it('lists_all_13_admin_panel_routes', () => {
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

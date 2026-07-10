/**
 * Contract: Phase 11 — every sidebar route is registered in TanStack Router.
 */
import { readFileSync } from 'node:fs';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

import { describe, expect, it } from 'vitest';

import { platformNavItems } from '@/lib/platform-nav';

const testDir = dirname(fileURLToPath(import.meta.url));
const routeTreeSource = readFileSync(resolve(testDir, '../../src/routeTree.gen.ts'), 'utf8');

describe('platform route coverage — Phase 11 contract', () => {
  it('registers_every_sidebar_route_in_route_tree', () => {
    for (const item of platformNavItems) {
      const normalized = item.to === '/' ? "'/'" : `'${item.to}'`;
      expect(routeTreeSource).toContain(normalized);
    }
  });

  it('lists_all_9_platform_panel_routes', () => {
    expect(platformNavItems).toHaveLength(9);
  });
});

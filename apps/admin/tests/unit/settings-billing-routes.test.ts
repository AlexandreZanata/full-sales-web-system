/**
 * Contract: Phase 12 — settings sub-routes registered in TanStack Router.
 */
import { readFileSync } from 'node:fs';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

import { describe, expect, it } from 'vitest';

const testDir = dirname(fileURLToPath(import.meta.url));
const routeTreeSource = readFileSync(resolve(testDir, '../../src/routeTree.gen.ts'), 'utf8');

const SETTINGS_ROUTES = ['/settings/billing', '/settings/payments', '/settings/domains'];

describe('settings billing routes — Phase 12 contract', () => {
  it('registers_billing_payments_domains_routes', () => {
    for (const route of SETTINGS_ROUTES) {
      expect(routeTreeSource).toContain(`'${route}'`);
    }
  });
});

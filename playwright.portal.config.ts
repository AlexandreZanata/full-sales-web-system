import { defineConfig, devices } from '@playwright/test';

const PORTAL_PORT = 5175;
const PORTAL_URL = `http://127.0.0.1:${PORTAL_PORT}`;

export default defineConfig({
  testDir: './e2e',
  testMatch: ['portal-order.spec.ts', 'portal-responsive.spec.ts', 'portal-catalog.spec.ts'],
  fullyParallel: true,
  forbidOnly: Boolean(process.env.CI),
  retries: process.env.CI ? 1 : 0,
  reporter: process.env.CI ? 'github' : 'list',
  use: {
    baseURL: PORTAL_URL,
    trace: 'on-first-retry',
  },
  projects: [{ name: 'chromium', use: { ...devices['Desktop Chrome'] } }],
  webServer: {
    command: 'pnpm --filter @full-sales/portal dev',
    url: PORTAL_URL,
    reuseExistingServer: !process.env.CI,
    timeout: 120_000,
  },
});

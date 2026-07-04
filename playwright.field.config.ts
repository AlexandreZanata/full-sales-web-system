import { defineConfig, devices } from '@playwright/test';

const FIELD_PORT = 5176;
const FIELD_URL = `http://127.0.0.1:${FIELD_PORT}`;

export default defineConfig({
  testDir: './e2e',
  testMatch: 'field-sale.spec.ts',
  fullyParallel: true,
  forbidOnly: Boolean(process.env.CI),
  retries: process.env.CI ? 1 : 0,
  reporter: process.env.CI ? 'github' : 'list',
  use: {
    baseURL: FIELD_URL,
    trace: 'on-first-retry',
  },
  projects: [{ name: 'chromium', use: { ...devices['Desktop Chrome'] } }],
  webServer: {
    command: 'pnpm --filter @full-sales/field dev',
    url: FIELD_URL,
    reuseExistingServer: !process.env.CI,
    timeout: 120_000,
  },
});

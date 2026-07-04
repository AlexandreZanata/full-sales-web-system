import { defineConfig, devices } from '@playwright/test';

const ADMIN_PORT = 5174;
const ADMIN_URL = `http://127.0.0.1:${ADMIN_PORT}`;

export default defineConfig({
  testDir: './e2e',
  fullyParallel: true,
  forbidOnly: Boolean(process.env.CI),
  retries: process.env.CI ? 1 : 0,
  reporter: process.env.CI ? 'github' : 'list',
  use: {
    baseURL: ADMIN_URL,
    trace: 'on-first-retry',
  },
  projects: [
    {
      name: 'chromium',
      use: { ...devices['Desktop Chrome'] },
    },
    {
      name: 'mobile-chrome',
      use: { ...devices['Pixel 5'] },
    },
  ],
  webServer: {
    command: 'pnpm --filter @full-sales/admin dev',
    url: ADMIN_URL,
    reuseExistingServer: !process.env.CI,
    timeout: 120_000,
  },
});

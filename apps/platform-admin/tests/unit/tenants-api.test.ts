/**
 * Contract: docs/API-CONTRACT.md — platform tenants list filters + PATCH + create user.
 */
import { afterEach, describe, expect, it, vi } from 'vitest';

import { fetchTenants, patchTenant } from '@/lib/api/tenants';
import { createPlatformTenantUser } from '@/lib/api/users';

describe('tenants API — Phase 11 contract', () => {
  afterEach(() => {
    vi.unstubAllGlobals();
  });

  it('builds_filter_status_query_param', async () => {
    const fetchMock = vi.fn().mockResolvedValue({
      ok: true,
      status: 200,
      json: async () => ({ data: [], meta: {} }),
    });
    vi.stubGlobal('fetch', fetchMock);

    await fetchTenants({ limit: 20, status: 'Suspended' });

    expect(fetchMock).toHaveBeenCalledWith(
      expect.stringContaining('filter%5Bstatus%5D=Suspended'),
      expect.anything(),
    );
  });

  it('patches_display_name_and_plan_id', async () => {
    const fetchMock = vi.fn().mockResolvedValue({
      ok: true,
      status: 200,
      json: async () => ({
        id: 't1',
        legalName: 'Acme',
        displayName: 'Acme Store',
        status: 'Active',
        planId: 'pro',
        counts: { users: 1, commerces: 0, orders: 0 },
        settings: {},
      }),
    });
    vi.stubGlobal('fetch', fetchMock);

    await patchTenant('t1', { displayName: 'Acme Store', planId: 'pro' });

    expect(fetchMock).toHaveBeenCalledWith(
      expect.stringContaining('/platform/tenants/t1'),
      expect.objectContaining({
        method: 'PATCH',
        body: JSON.stringify({ displayName: 'Acme Store', planId: 'pro' }),
      }),
    );
  });

  it('creates_platform_tenant_user', async () => {
    const fetchMock = vi.fn().mockResolvedValue({
      ok: true,
      status: 201,
      json: async () => ({
        user: {
          id: 'u1',
          tenantId: 't1',
          tenant: { id: 't1', displayName: 'Demo' },
          email: 'a@b.com',
          name: 'Ada Lovelace',
          role: 'Admin',
          active: true,
          createdAt: '2026-01-01T00:00:00Z',
        },
        temporaryPassword: 'temp',
      }),
    });
    vi.stubGlobal('fetch', fetchMock);

    await createPlatformTenantUser('t1', {
      name: 'Ada Lovelace',
      email: 'a@b.com',
      role: 'Admin',
    });

    expect(fetchMock).toHaveBeenCalledWith(
      expect.stringContaining('/platform/tenants/t1/users'),
      expect.objectContaining({
        method: 'POST',
        body: JSON.stringify({
          name: 'Ada Lovelace',
          email: 'a@b.com',
          role: 'Admin',
        }),
      }),
    );
  });
});

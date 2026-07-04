/**
 * Contract: Admin session restore — Admin role required (Phase 29).
 */
import { afterEach, describe, expect, it } from 'vitest';

import { resolveSessionFromAccessToken } from '@/lib/auth/authSession';
import { DEV_STUB_ACCESS } from '@/lib/auth/devStub';
import { clearStoredUserEmail, setStoredUserEmail } from '@/lib/auth/sessionUser';
import { clearTokens, setTokens } from '@/lib/auth/tokens';

function makeJwt(payload: Record<string, unknown>): string {
  const header = btoa(JSON.stringify({ alg: 'HS256', typ: 'JWT' }));
  const body = btoa(JSON.stringify(payload));
  return `${header}.${body}.signature`;
}

describe('resolveSessionFromAccessToken — Admin gate contract', () => {
  afterEach(() => {
    clearTokens();
    clearStoredUserEmail();
  });

  it('given_admin_token_and_email_when_restore_then_ok', () => {
    const token = makeJwt({
      sub: '550e8400-e29b-41d4-a716-446655440000',
      tenant_id: '660e8400-e29b-41d4-a716-446655440001',
      role: 'Admin',
      exp: Math.floor(Date.now() / 1000) + 900,
    });
    setStoredUserEmail('admin@test.com');

    const result = resolveSessionFromAccessToken(token);
    expect(result.kind).toBe('ok');
    if (result.kind === 'ok') {
      expect(result.user).toEqual({ email: 'admin@test.com', role: 'Admin' });
    }
  });

  it('given_driver_token_when_restore_then_invalid', () => {
    const token = makeJwt({
      sub: '550e8400-e29b-41d4-a716-446655440000',
      tenant_id: '660e8400-e29b-41d4-a716-446655440001',
      role: 'Driver',
      exp: Math.floor(Date.now() / 1000) + 900,
    });
    setStoredUserEmail('driver@test.com');

    expect(resolveSessionFromAccessToken(token).kind).toBe('invalid');
  });

  it('given_dev_stub_without_email_when_restore_then_missing', () => {
    setTokens({ accessToken: DEV_STUB_ACCESS, refreshToken: 'dev-stub-refresh' });
    expect(resolveSessionFromAccessToken(DEV_STUB_ACCESS).kind).toBe('missing');
  });

  it('given_expired_admin_token_when_restore_then_needs_refresh', () => {
    const token = makeJwt({
      sub: '550e8400-e29b-41d4-a716-446655440000',
      tenant_id: '660e8400-e29b-41d4-a716-446655440001',
      role: 'Admin',
      exp: 1,
    });
    setStoredUserEmail('admin@test.com');

    const result = resolveSessionFromAccessToken(token);
    expect(result.kind).toBe('ok');
    if (result.kind === 'ok') {
      expect(result.needsRefresh).toBe(true);
    }
  });
});

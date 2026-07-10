/**
 * Contract: Platform JWT — role PlatformAdmin (backend infra-crypto PlatformAccessTokenClaims).
 */
import { describe, expect, it } from 'vitest';

import { decodeAccessTokenClaims, isPlatformAdminRole, isTokenExpired } from '@/lib/auth/jwt';

function makeJwt(payload: Record<string, unknown>): string {
  const header = btoa(JSON.stringify({ alg: 'HS256', typ: 'JWT' }));
  const body = btoa(JSON.stringify(payload));
  return `${header}.${body}.signature`;
}

describe('platform JWT — Phase 11 contract', () => {
  it('given_platform_admin_claims_when_decode_then_role', () => {
    const token = makeJwt({
      sub: '550e8400-e29b-41d4-a716-446655440000',
      role: 'PlatformAdmin',
      exp: Math.floor(Date.now() / 1000) + 900,
    });
    const claims = decodeAccessTokenClaims(token);
    expect(claims?.role).toBe('PlatformAdmin');
    expect(isPlatformAdminRole(claims?.role ?? '')).toBe(true);
  });

  it('given_tenant_admin_role_when_isPlatformAdminRole_then_false', () => {
    expect(isPlatformAdminRole('Admin')).toBe(false);
  });

  it('given_expired_claims_when_isTokenExpired_then_true', () => {
    const claims = decodeAccessTokenClaims(
      makeJwt({ sub: '550e8400-e29b-41d4-a716-446655440000', role: 'PlatformAdmin', exp: 1 }),
    );
    expect(claims).not.toBeNull();
    if (!claims) {
      throw new Error('expected claims');
    }
    expect(isTokenExpired(claims, 2_000)).toBe(true);
  });
});

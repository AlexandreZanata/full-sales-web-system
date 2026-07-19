/**
 * Contract: JWT access token claims — role, exp (backend infra-crypto AccessTokenClaims).
 */
import { describe, expect, it } from 'vitest';

import {
  decodeAccessTokenClaims,
  isAdminRole,
  isTokenExpired,
  tokenExpiresWithinMs,
} from '@/lib/auth/jwt';

function makeJwt(payload: Record<string, unknown>): string {
  const header = btoa(JSON.stringify({ alg: 'HS256', typ: 'JWT' }));
  const body = btoa(JSON.stringify(payload));
  return `${header}.${body}.signature`;
}

describe('decodeAccessTokenClaims — JWT payload contract', () => {
  it('given_admin_claims_when_decode_then_role_and_exp', () => {
    const exp = Math.floor(Date.now() / 1000) + 900;
    const token = makeJwt({
      sub: '550e8400-e29b-41d4-a716-446655440000',
      tenant_id: '660e8400-e29b-41d4-a716-446655440001',
      role: 'Admin',
      exp,
    });

    const claims = decodeAccessTokenClaims(token);
    expect(claims).toMatchObject({ role: 'Admin', exp });
    expect(isAdminRole(claims?.role ?? '')).toBe(true);
  });

  it('given_driver_role_when_isAdminRole_then_false', () => {
    expect(isAdminRole('Driver')).toBe(false);
  });

  it('given_expired_claims_when_isTokenExpired_then_true', () => {
    const claims = decodeAccessTokenClaims(
      makeJwt({
        sub: '550e8400-e29b-41d4-a716-446655440000',
        tenant_id: '660e8400-e29b-41d4-a716-446655440001',
        role: 'Admin',
        exp: 1,
      }),
    );
    expect(claims).not.toBeNull();
    if (!claims) {
      throw new Error('expected claims');
    }
    expect(isTokenExpired(claims, 2_000)).toBe(true);
  });

  it('given_token_expiring_soon_when_tokenExpiresWithinMs_then_true', () => {
    const now = 1_000_000;
    const claims = decodeAccessTokenClaims(
      makeJwt({
        sub: '550e8400-e29b-41d4-a716-446655440000',
        tenant_id: '660e8400-e29b-41d4-a716-446655440001',
        role: 'Admin',
        exp: Math.floor((now + 30_000) / 1000),
      }),
    );
    if (!claims) {
      throw new Error('expected claims');
    }
    expect(tokenExpiresWithinMs(claims, 60_000, now)).toBe(true);
  });

  it('given_platform_impersonation_jwt_when_decode_then_tenant_admin_claims', () => {
    const exp = Math.floor(Date.now() / 1000) + 900;
    const token = makeJwt({
      sub: '550e8400-e29b-41d4-a716-446655440000',
      role: 'PlatformAdmin',
      exp,
      impersonating: true,
      actingTenantId: '660e8400-e29b-41d4-a716-446655440001',
      actingRole: 'Admin',
      actingUserId: '770e8400-e29b-41d4-a716-446655440002',
    });

    const claims = decodeAccessTokenClaims(token);
    expect(claims).toMatchObject({
      role: 'Admin',
      tenant_id: '660e8400-e29b-41d4-a716-446655440001',
      sub: '770e8400-e29b-41d4-a716-446655440002',
      impersonating: true,
      exp,
    });
    expect(isAdminRole(claims?.role ?? '')).toBe(true);
  });
});

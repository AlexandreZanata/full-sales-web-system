export type PlatformAccessTokenClaims = {
  sub: string;
  role: string;
  exp: number;
  impersonating?: boolean;
  actingTenantId?: string;
  actingRole?: string;
  grantId?: string;
  actingUserId?: string;
};

export function decodeAccessTokenClaims(token: string): PlatformAccessTokenClaims | null {
  const parts = token.split('.');
  if (parts.length !== 3) {
    return null;
  }

  try {
    const payload = parts[1].replace(/-/g, '+').replace(/_/g, '/');
    const padded = payload.padEnd(payload.length + ((4 - (payload.length % 4)) % 4), '=');
    const decoded = JSON.parse(atob(padded)) as Partial<PlatformAccessTokenClaims>;
    if (
      typeof decoded.sub !== 'string' ||
      typeof decoded.role !== 'string' ||
      typeof decoded.exp !== 'number'
    ) {
      return null;
    }
    return decoded as PlatformAccessTokenClaims;
  } catch {
    return null;
  }
}

export function isPlatformAdminRole(role: string): boolean {
  return role === 'PlatformAdmin';
}

export function isTokenExpired(claims: PlatformAccessTokenClaims, nowMs = Date.now()): boolean {
  return claims.exp * 1000 <= nowMs;
}

export function tokenExpiresWithinMs(
  claims: PlatformAccessTokenClaims,
  withinMs: number,
  nowMs = Date.now(),
): boolean {
  return claims.exp * 1000 - nowMs <= withinMs;
}

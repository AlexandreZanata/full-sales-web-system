export type AccessTokenClaims = {
  sub: string;
  tenant_id: string;
  role: string;
  commerceId?: string;
  exp: number;
  impersonating?: boolean;
};

type RawClaims = Partial<AccessTokenClaims> & {
  actingTenantId?: string;
  actingRole?: string;
  actingUserId?: string;
  impersonating?: boolean;
  role?: string;
  sub?: string;
  exp?: number;
};

function decodePayload(token: string): RawClaims | null {
  const parts = token.split('.');
  if (parts.length !== 3) {
    return null;
  }
  try {
    const payload = parts[1].replace(/-/g, '+').replace(/_/g, '/');
    const padded = payload.padEnd(payload.length + ((4 - (payload.length % 4)) % 4), '=');
    return JSON.parse(atob(padded)) as RawClaims;
  } catch {
    return null;
  }
}

/** Contract: tenant Admin JWT, or platform impersonation JWT (SECURITY.md). */
export function decodeAccessTokenClaims(token: string): AccessTokenClaims | null {
  const decoded = decodePayload(token);
  if (!decoded || typeof decoded.exp !== 'number' || typeof decoded.sub !== 'string') {
    return null;
  }

  if (decoded.impersonating === true && typeof decoded.actingTenantId === 'string') {
    const role =
      typeof decoded.actingRole === 'string' && decoded.actingRole.length > 0
        ? decoded.actingRole
        : 'Admin';
    const sub =
      typeof decoded.actingUserId === 'string' && decoded.actingUserId.length > 0
        ? decoded.actingUserId
        : decoded.sub;
    return {
      sub,
      tenant_id: decoded.actingTenantId,
      role,
      exp: decoded.exp,
      impersonating: true,
    };
  }

  if (
    typeof decoded.tenant_id !== 'string' ||
    typeof decoded.role !== 'string'
  ) {
    return null;
  }

  return {
    sub: decoded.sub,
    tenant_id: decoded.tenant_id,
    role: decoded.role,
    commerceId: decoded.commerceId,
    exp: decoded.exp,
  };
}

export function isAdminRole(role: string): boolean {
  return role === 'Admin';
}

export function isTokenExpired(claims: AccessTokenClaims, nowMs = Date.now()): boolean {
  return claims.exp * 1000 <= nowMs;
}

export function tokenExpiresWithinMs(
  claims: AccessTokenClaims,
  withinMs: number,
  nowMs = Date.now(),
): boolean {
  return claims.exp * 1000 - nowMs <= withinMs;
}

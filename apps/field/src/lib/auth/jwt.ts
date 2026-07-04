export type AccessTokenClaims = {
  sub: string;
  tenant_id: string;
  role: string;
  commerceId?: string;
  exp: number;
};

export function decodeAccessTokenClaims(token: string): AccessTokenClaims | null {
  const parts = token.split('.');
  if (parts.length !== 3) return null;
  try {
    const payload = parts[1].replace(/-/g, '+').replace(/_/g, '/');
    const padded = payload.padEnd(payload.length + ((4 - (payload.length % 4)) % 4), '=');
    const decoded = JSON.parse(atob(padded)) as Partial<AccessTokenClaims>;
    if (
      typeof decoded.sub !== 'string' ||
      typeof decoded.tenant_id !== 'string' ||
      typeof decoded.role !== 'string' ||
      typeof decoded.exp !== 'number'
    ) {
      return null;
    }
    return decoded as AccessTokenClaims;
  } catch {
    return null;
  }
}

export function isFieldRole(role: string): boolean {
  return role === 'Driver' || role === 'Seller';
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

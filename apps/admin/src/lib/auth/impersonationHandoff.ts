import { setStoredUserEmail } from '@/lib/auth/sessionUser';
import { decodeAccessTokenClaims } from '@/lib/auth/jwt';
import { setTokens } from '@/lib/auth/tokens';

const HANDOFF_PREFIX = '#impersonation=';
const IMPERSONATION_REFRESH = 'impersonation';

/** Consumes `#impersonation=<jwt>` from platform-admin handoff (SECURITY.md). */
export function consumeImpersonationHandoff(hash = window.location.hash): boolean {
  if (!hash.startsWith(HANDOFF_PREFIX)) {
    return false;
  }

  const token = decodeURIComponent(hash.slice(HANDOFF_PREFIX.length));
  const claims = decodeAccessTokenClaims(token);
  if (!claims?.impersonating || !claims.tenant_id) {
    return false;
  }

  const expiresIn = Math.max(60, claims.exp - Math.floor(Date.now() / 1000));
  setTokens({
    accessToken: token,
    refreshToken: IMPERSONATION_REFRESH,
    expiresIn,
  });
  setStoredUserEmail('platform-impersonation@local');
  window.history.replaceState(null, '', `${window.location.pathname}${window.location.search}`);
  return true;
}

export function isImpersonationRefreshToken(refreshToken: string | null): boolean {
  return refreshToken === IMPERSONATION_REFRESH;
}

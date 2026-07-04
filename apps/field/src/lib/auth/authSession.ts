import { isDevStubAccessToken } from '@/lib/auth/devStub';
import {
  decodeAccessTokenClaims,
  isFieldRole,
  isTokenExpired,
  tokenExpiresWithinMs,
  type AccessTokenClaims,
} from '@/lib/auth/jwt';
import { getStoredUserEmail } from '@/lib/auth/sessionUser';

export type SessionUser = {
  email: string;
  role: string;
};

export type SessionRestoreResult =
  | { kind: 'ok'; user: SessionUser; claims: AccessTokenClaims | null; needsRefresh: boolean }
  | { kind: 'missing' }
  | { kind: 'invalid' };

const REFRESH_LEAD_MS = 60_000;

export function resolveSessionFromAccessToken(accessToken: string): SessionRestoreResult {
  const email = getStoredUserEmail();

  if (isDevStubAccessToken(accessToken)) {
    if (!email) return { kind: 'missing' };
    return {
      kind: 'ok',
      user: { email, role: 'Seller' },
      claims: null,
      needsRefresh: false,
    };
  }

  const claims = decodeAccessTokenClaims(accessToken);
  if (!claims || !email) return { kind: 'invalid' };
  if (!isFieldRole(claims.role)) return { kind: 'invalid' };

  const needsRefresh = isTokenExpired(claims) || tokenExpiresWithinMs(claims, REFRESH_LEAD_MS);

  return {
    kind: 'ok',
    user: { email, role: claims.role },
    claims,
    needsRefresh,
  };
}

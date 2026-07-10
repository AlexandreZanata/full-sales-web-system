import { isDevStubAccessToken } from '@/lib/auth/devStub';
import {
  decodeAccessTokenClaims,
  isPlatformAdminRole,
  isTokenExpired,
  tokenExpiresWithinMs,
  type PlatformAccessTokenClaims,
} from '@/lib/auth/jwt';
import {
  clearStoredImpersonation,
  getStoredImpersonation,
  getStoredUserEmail,
  setStoredImpersonation,
} from '@/lib/auth/sessionUser';

export type SessionUser = {
  email: string;
  role: string;
  impersonating: boolean;
  actingTenantId?: string;
};

export type SessionRestoreResult =
  | {
      kind: 'ok';
      user: SessionUser;
      claims: PlatformAccessTokenClaims | null;
      needsRefresh: boolean;
    }
  | { kind: 'missing' }
  | { kind: 'invalid' };

const REFRESH_LEAD_MS = 60_000;

function syncImpersonationFromClaims(claims: PlatformAccessTokenClaims | null): void {
  if (claims?.impersonating && claims.actingTenantId) {
    setStoredImpersonation({
      tenantId: claims.actingTenantId,
      grantId: claims.grantId,
    });
    return;
  }
  if (!getStoredImpersonation()) {
    clearStoredImpersonation();
  }
}

export function resolveSessionFromAccessToken(accessToken: string): SessionRestoreResult {
  const email = getStoredUserEmail();

  if (isDevStubAccessToken(accessToken)) {
    if (!email) {
      return { kind: 'missing' };
    }
    const stored = getStoredImpersonation();
    return {
      kind: 'ok',
      user: {
        email,
        role: 'PlatformAdmin',
        impersonating: Boolean(stored),
        actingTenantId: stored?.tenantId,
      },
      claims: null,
      needsRefresh: false,
    };
  }

  const claims = decodeAccessTokenClaims(accessToken);
  if (!claims || !email) {
    return { kind: 'invalid' };
  }

  if (!isPlatformAdminRole(claims.role)) {
    return { kind: 'invalid' };
  }

  syncImpersonationFromClaims(claims);
  const needsRefresh = isTokenExpired(claims) || tokenExpiresWithinMs(claims, REFRESH_LEAD_MS);

  return {
    kind: 'ok',
    user: {
      email,
      role: claims.role,
      impersonating: Boolean(claims.impersonating),
      actingTenantId: claims.actingTenantId,
    },
    claims,
    needsRefresh,
  };
}

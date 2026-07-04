import { isDevStubAccessToken } from '@/lib/auth/devStub';
import { decodeAccessTokenClaims } from '@/lib/auth/jwt';
import { getAccessToken } from '@/lib/auth/tokens';

/** Dev tenant UUID from `pnpm seed:dev` — used when stub token has no JWT claims. */
export const DEV_TENANT_ID = '01900001-0000-7000-8000-000000000001';

export function resolveTenantIdFromSession(): string {
  const token = getAccessToken();
  if (!token) {
    return '';
  }
  if (isDevStubAccessToken(token)) {
    return DEV_TENANT_ID;
  }
  return decodeAccessTokenClaims(token)?.tenant_id ?? '';
}

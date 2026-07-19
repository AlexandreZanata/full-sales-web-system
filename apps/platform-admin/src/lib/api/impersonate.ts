import { apiPost } from '@/lib/api/client';
import { decodeAccessTokenClaims } from '@/lib/auth/jwt';
import { setStoredImpersonation } from '@/lib/auth/sessionUser';

export type ImpersonationStartResult = {
  impersonationToken: string;
  expiresAt: string;
  tenantId: string;
  grantId?: string;
};

const ADMIN_ORIGIN = import.meta.env.VITE_ADMIN_ORIGIN ?? 'http://127.0.0.1:5174';

export async function startImpersonation(body: {
  tenantId: string;
  userId?: string;
  reason: string;
}): Promise<ImpersonationStartResult> {
  const response = await apiPost<{
    impersonationToken: string;
    expiresAt: string;
    tenantId: string;
  }>('/platform/impersonate', body);

  const claims = decodeAccessTokenClaims(response.impersonationToken);
  const grantId = claims?.grantId;

  setStoredImpersonation({
    tenantId: response.tenantId,
    grantId,
  });

  return {
    ...response,
    grantId,
  };
}

export function openTenantAdminWithImpersonation(token: string): void {
  const url = `${ADMIN_ORIGIN}/#impersonation=${encodeURIComponent(token)}`;
  window.open(url, '_blank', 'noopener,noreferrer');
}

export async function endImpersonation(grantId: string): Promise<void> {
  await apiPost('/platform/impersonate/end', { grantId });
  setStoredImpersonation(null);
}

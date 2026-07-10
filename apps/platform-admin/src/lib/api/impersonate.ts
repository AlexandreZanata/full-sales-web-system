import { apiPost } from '@/lib/api/client';
import { setStoredImpersonation } from '@/lib/auth/sessionUser';

export async function startImpersonation(body: {
  tenantId: string;
  userId?: string;
  reason: string;
}): Promise<{ impersonationToken: string; expiresAt: string; tenantId: string }> {
  const response = await apiPost<{
    impersonationToken: string;
    expiresAt: string;
    tenantId: string;
  }>('/platform/impersonate', body);
  setStoredImpersonation({ tenantId: response.tenantId, grantId: undefined });
  return response;
}

export async function endImpersonation(grantId: string): Promise<void> {
  await apiPost('/platform/impersonate/end', { grantId });
  setStoredImpersonation(null);
}

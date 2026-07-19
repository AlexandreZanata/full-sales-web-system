import { resolveSessionFromAccessToken } from '@/lib/auth/authSession';
import {
  consumeImpersonationHandoff,
  isImpersonationRefreshToken,
} from '@/lib/auth/impersonationHandoff';
import { clearTokens, getAccessToken, getRefreshToken, hasSession } from '@/lib/auth/tokens';
import { tryRefreshTokens } from '@/lib/api/client';

export type RestoredAdminUser = { email: string; role: string };

export async function restoreAdminSession(): Promise<RestoredAdminUser | null> {
  consumeImpersonationHandoff();

  if (!hasSession()) {
    return null;
  }

  const accessToken = getAccessToken();
  if (!accessToken) {
    clearTokens();
    return null;
  }

  const restored = resolveSessionFromAccessToken(accessToken);
  if (restored.kind === 'missing' || restored.kind === 'invalid') {
    clearTokens();
    return null;
  }

  if (restored.needsRefresh && !isImpersonationRefreshToken(getRefreshToken())) {
    const refreshed = await tryRefreshTokens();
    if (!refreshed) {
      return null;
    }
    const token = getAccessToken();
    if (!token) {
      return null;
    }
    const afterRefresh = resolveSessionFromAccessToken(token);
    if (afterRefresh.kind !== 'ok') {
      clearTokens();
      return null;
    }
    return afterRefresh.user;
  }

  return restored.user;
}

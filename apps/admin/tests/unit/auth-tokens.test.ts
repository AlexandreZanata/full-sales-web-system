/**
 * Contract: sessionStorage-backed token persistence (Phase 28/29 auth storage).
 */
import { afterEach, describe, expect, it } from 'vitest';

import {
  clearStoredUserEmail,
  getStoredUserEmail,
  setStoredUserEmail,
} from '@/lib/auth/sessionUser';
import {
  clearTokens,
  getAccessToken,
  getRefreshToken,
  getTokenExpiresAt,
  hasSession,
  msUntilTokenRefresh,
  setTokens,
} from '@/lib/auth/tokens';

describe('auth tokens — persistence contract', () => {
  afterEach(() => {
    clearTokens();
    clearStoredUserEmail();
  });

  it('given_tokens_when_set_then_hasSession_and_reads_back', () => {
    setTokens({
      accessToken: 'access-1',
      refreshToken: 'refresh-1',
      expiresIn: 900,
    });

    expect(hasSession()).toBe(true);
    expect(getAccessToken()).toBe('access-1');
    expect(getRefreshToken()).toBe('refresh-1');
    expect(getTokenExpiresAt()).toBeGreaterThan(Date.now());
  });

  it('given_clear_when_called_then_session_removed', () => {
    setTokens({ accessToken: 'access-1', refreshToken: 'refresh-1' });
    clearTokens();

    expect(hasSession()).toBe(false);
    expect(getAccessToken()).toBeNull();
    expect(getStoredUserEmail()).toBeNull();
  });

  it('given_expiresAt_when_msUntilTokenRefresh_then_lead_subtracted', () => {
    const expiresAt = Date.now() + 120_000;
    const delay = msUntilTokenRefresh(expiresAt, 60_000);
    expect(delay).toBeGreaterThan(50_000);
    expect(delay).toBeLessThanOrEqual(60_000);
  });

  it('given_email_when_stored_then_read_back', () => {
    setStoredUserEmail('admin@test.com');
    expect(getStoredUserEmail()).toBe('admin@test.com');
  });
});

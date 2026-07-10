export const DEV_STUB_ACCESS = 'dev-stub-platform-access';
export const DEV_STUB_REFRESH = 'dev-stub-platform-refresh';
export const DEV_STUB_EMAIL = 'platform@localhost';

export function isDevStubAccessToken(token: string): boolean {
  return token === DEV_STUB_ACCESS;
}

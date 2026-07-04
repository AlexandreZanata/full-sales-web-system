export const DEV_STUB_ACCESS = 'portal-dev-stub-access';
export const DEV_STUB_REFRESH = 'portal-dev-stub-refresh';
export const DEV_STUB_EMAIL = 'portal@seed-store.com';

export function isDevStubAccessToken(token: string): boolean {
  return token === DEV_STUB_ACCESS;
}

export const DEV_STUB_ACCESS = 'dev-stub-access';
export const DEV_STUB_REFRESH = 'dev-stub-refresh';
export const DEV_STUB_EMAIL = 'admin@localhost';

export function isDevStubAccessToken(token: string): boolean {
  return token === DEV_STUB_ACCESS;
}

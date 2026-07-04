export const DEV_STUB_ACCESS = 'field-dev-stub-access';
export const DEV_STUB_REFRESH = 'field-dev-stub-refresh';
export const DEV_STUB_EMAIL = 'seller@test.com';

export function isDevStubAccessToken(token: string): boolean {
  return token === DEV_STUB_ACCESS;
}

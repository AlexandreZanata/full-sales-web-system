function encodeJwtPayload(payload: Record<string, unknown>): string {
  return Buffer.from(JSON.stringify(payload)).toString('base64url');
}

export function buildPortalAccessToken(email = 'portal@seed-store.com'): string {
  const exp = Math.floor(Date.now() / 1000) + 3600;
  const payload = encodeJwtPayload({
    sub: '01900001-0004-7000-8000-000000000001',
    tenant_id: '01900001-0000-7000-8000-000000000001',
    role: 'CommerceContact',
    commerceId: '01900001-0010-7000-8000-000000000001',
    exp,
  });
  return `e2e.${payload}.sig`;
}

export function buildFieldAccessToken(email = 'seller@test.com', role = 'Seller'): string {
  const exp = Math.floor(Date.now() / 1000) + 3600;
  const payload = encodeJwtPayload({
    sub: '01900001-0003-7000-8000-000000000001',
    tenant_id: '01900001-0000-7000-8000-000000000001',
    role,
    exp,
  });
  return `e2e.${payload}.sig`;
}

export function loginResponse(accessToken: string) {
  return {
    accessToken,
    refreshToken: 'e2e-refresh-token',
    expiresIn: 3600,
  };
}

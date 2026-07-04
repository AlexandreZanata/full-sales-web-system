function encodeJwtPayload(payload: Record<string, unknown>): string {
  return Buffer.from(JSON.stringify(payload)).toString('base64url');
}

export function buildAdminAccessToken(email = 'admin@test.com'): string {
  const exp = Math.floor(Date.now() / 1000) + 3600;
  const payload = encodeJwtPayload({
    sub: '550e8400-e29b-41d4-a716-446655440000',
    tenant_id: '660e8400-e29b-41d4-a716-446655440001',
    role: 'Admin',
    exp,
  });
  return `e2e.${payload}.sig`;
}

export function loginResponse(email = 'admin@test.com') {
  const accessToken = buildAdminAccessToken(email);
  return {
    accessToken,
    refreshToken: 'e2e-refresh-token',
    expiresIn: 3600,
  };
}

export type { CursorListResponse } from '@/lib/cursorPagination';

export type TokenResponse = {
  accessToken: string;
  refreshToken: string;
  expiresIn?: number;
};

export type PlatformLoginResponse = { mfaRequired: true; mfaToken: string } | TokenResponse;

export type TenantListItem = {
  id: string;
  legalName: string;
  displayName: string;
  status: string;
  planId?: string;
  createdAt: string;
};

export type TenantDetail = {
  id: string;
  legalName: string;
  displayName: string;
  status: string;
  planId?: string;
  trialEndsAt?: string;
  suspendedAt?: string;
  suspendedReason?: string;
  counts: { users: number; commerces: number; orders: number };
  settings: Record<string, unknown>;
};

export type ProvisionTenantResponse = {
  tenantId: string;
  adminUserId: string;
  status: string;
  trialEndsAt?: string;
  adminTemporaryPassword: string;
};

export type PlatformUser = {
  id: string;
  tenantId: string;
  tenant: { id: string; displayName: string };
  email: string;
  name: string;
  role: string;
  active: boolean;
  createdAt: string;
  lastLoginAt?: string;
};

export type TenantStats = {
  users: number;
  drivers: number;
  sellers: number;
  commerces: number;
  orders: number;
  mrrMinor: number;
  mrrCurrency: string;
};

export type FraudEvent = {
  id: string;
  tenantId?: string;
  userId?: string;
  eventType: string;
  severity: string;
  status: string;
  resolution?: string;
  resolutionNote?: string;
  metadata: Record<string, unknown>;
  reviewedBy?: string;
  reviewedAt?: string;
  createdAt: string;
};

export type PlatformDomain = {
  id: string;
  tenantId: string;
  hostname: string;
  status: string;
  verifiedAt?: string;
  createdAt: string;
};

export type ProbeMatrixEntry = {
  status: string;
  latencyMs?: number;
  checkedAt?: string;
  uptime24hPct: number;
  details: Record<string, unknown>;
};

export type HealthMatrixResponse = {
  probes: Record<string, ProbeMatrixEntry>;
};

export type HealthHistoryPoint = {
  checkedAt: string;
  status: string;
  latencyMs?: number;
};

export type AuditEvent = {
  id: string;
  actorId: string;
  actorType: string;
  tenantId?: string;
  action: string;
  resource: string;
  resourceId: string;
  metadata?: Record<string, unknown>;
  ip?: string;
  createdAt: string;
};

export type MaintenanceWindow = {
  id: string;
  tenantId?: string;
  message: string;
  startsAt: string;
  endsAt: string;
};

export type BlocklistEntry = {
  id: string;
  kind: string;
  value: string;
  reason?: string;
  createdAt: string;
};

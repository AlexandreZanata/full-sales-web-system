import { apiDelete, apiFetch, apiPost } from '@/lib/api/client';

export type TenantDomain = {
  id: string;
  hostname: string;
  status: string;
  isPrimary: boolean;
  verifiedAt?: string;
  createdAt: string;
};

export type DomainListResponse = {
  data: TenantDomain[];
};

export type CreateDomainResponse = TenantDomain & {
  txtRecord: string;
  txtValue: string;
};

export type VerifyDomainResponse = {
  status: string;
  txtRecord: string;
  txtValue: string;
  verifiedAt?: string;
};

export async function fetchDomains(): Promise<DomainListResponse> {
  return apiFetch<DomainListResponse>('/settings/domains');
}

export async function createDomain(hostname: string): Promise<CreateDomainResponse> {
  return apiPost<CreateDomainResponse>('/settings/domains', { hostname });
}

export async function verifyDomain(id: string): Promise<VerifyDomainResponse> {
  return apiFetch<VerifyDomainResponse>(`/settings/domains/${id}/verify`);
}

export async function deleteDomain(id: string): Promise<void> {
  await apiDelete(`/settings/domains/${id}`);
}

export async function setPrimaryDomain(id: string): Promise<TenantDomain> {
  return apiPost<TenantDomain>(`/settings/domains/${id}/set-primary`, {});
}

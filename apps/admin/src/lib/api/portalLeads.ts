import { apiFetch } from '@/lib/api/client';

export type PortalLeadStatus = 'pending' | 'approved' | 'rejected';

export type PortalLead = {
  id: string;
  contactName: string;
  phone: string;
  commerceName: string;
  email: string;
  status: PortalLeadStatus;
  createdAt: string;
  reviewedAt?: string;
};

export async function fetchPortalLeads(status?: PortalLeadStatus): Promise<PortalLead[]> {
  const qs = status ? `?status=${encodeURIComponent(status)}` : '';
  return apiFetch<PortalLead[]>(`/commerces/portal-leads${qs}`);
}

export async function reviewPortalLead(
  id: string,
  status: 'approved' | 'rejected',
): Promise<PortalLead> {
  return apiFetch<PortalLead>(`/commerces/portal-leads/${id}`, {
    method: 'PATCH',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ status }),
  });
}

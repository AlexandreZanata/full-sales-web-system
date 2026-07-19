import { apiPost } from '@/lib/api/client';

export type CreateCommerceLeadRequest = {
  contactName: string;
  phone: string;
  commerceName: string;
  email: string;
};

export type CommerceLeadResponse = {
  id: string;
  status: string;
};

export async function submitCommerceLead(
  body: CreateCommerceLeadRequest,
): Promise<CommerceLeadResponse> {
  return apiPost<CommerceLeadResponse>('/public/commerce-leads', body, { skipAuth: true });
}

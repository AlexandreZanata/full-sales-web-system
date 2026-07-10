import { apiFetch, apiPost } from '@/lib/api/client';

export type PlanSummary = {
  id: string;
  code: string;
  name: string;
  priceMinor: number;
  billingInterval: string;
};

export type Subscription = {
  plan: PlanSummary;
  status: string;
  tenantStatus: string;
  currentPeriodEnd?: string;
  trialEndsAt?: string;
};

export type InvoiceSummary = {
  id: string;
  amountMinor: number;
  currency: string;
  dueDate: string;
  status: string;
  paidAt?: string;
};

export type InvoiceDetail = InvoiceSummary & {
  pdfUrl?: string;
};

export type InvoiceListResponse = {
  data: InvoiceSummary[];
  cursor?: string;
};

export async function fetchSubscription(): Promise<Subscription> {
  return apiFetch<Subscription>('/billing/subscription');
}

export async function fetchInvoices(
  params: {
    limit?: number;
    cursor?: string;
  } = {},
): Promise<InvoiceListResponse> {
  const search = new URLSearchParams();
  if (params.limit) {
    search.set('limit', String(params.limit));
  }
  if (params.cursor) {
    search.set('cursor', params.cursor);
  }
  const query = search.toString();
  return apiFetch<InvoiceListResponse>(`/billing/invoices${query ? `?${query}` : ''}`);
}

export async function fetchInvoice(id: string): Promise<InvoiceDetail> {
  return apiFetch<InvoiceDetail>(`/billing/invoices/${id}`);
}

export async function attachPaymentMethod(creditCardToken: string): Promise<void> {
  await apiPost('/billing/payment-methods', {
    type: 'credit_card',
    creditCardToken,
  });
}

export async function cancelSubscription(): Promise<void> {
  await apiPost('/billing/subscription/cancel', {});
}

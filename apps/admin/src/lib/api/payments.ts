import { apiDelete, apiFetch, apiPost, apiPut } from '@/lib/api/client';

export type PaymentMethods = {
  pix: boolean;
  credit: boolean;
  boleto: boolean;
};

export type PaymentSettings = {
  enabled: boolean;
  methods: PaymentMethods;
  autoCapture: boolean;
  asaas: {
    connected: boolean;
    apiKeyLast4?: string;
    connectedAt?: string;
  };
};

export type PaymentBalance = {
  balanceMinor: number;
  currency: string;
};

export type PaymentTransaction = {
  id: string;
  type: string;
  amountMinor: number;
  date: string;
  description?: string;
};

export type TransactionsResponse = {
  data: PaymentTransaction[];
  hasMore: boolean;
};

export async function fetchPaymentSettings(): Promise<PaymentSettings> {
  return apiFetch<PaymentSettings>('/settings/payments');
}

export async function updatePaymentSettings(body: {
  enabled: boolean;
  methods: PaymentMethods;
  autoCapture: boolean;
}): Promise<PaymentSettings> {
  return apiPut<PaymentSettings>('/settings/payments', body);
}

export async function connectAsaas(
  apiKey: string,
): Promise<{ connected: boolean; accountName: string }> {
  return apiPost('/settings/payments/asaas/connect', { apiKey });
}

export async function disconnectAsaas(): Promise<void> {
  await apiDelete('/settings/payments/asaas/connect');
}

export async function fetchPaymentBalance(): Promise<PaymentBalance> {
  return apiFetch<PaymentBalance>('/settings/payments/balance');
}

export async function fetchPaymentTransactions(
  params: {
    offset?: number;
    limit?: number;
  } = {},
): Promise<TransactionsResponse> {
  const search = new URLSearchParams();
  if (params.offset !== undefined) {
    search.set('offset', String(params.offset));
  }
  if (params.limit) {
    search.set('limit', String(params.limit));
  }
  const query = search.toString();
  return apiFetch<TransactionsResponse>(
    `/settings/payments/transactions${query ? `?${query}` : ''}`,
  );
}

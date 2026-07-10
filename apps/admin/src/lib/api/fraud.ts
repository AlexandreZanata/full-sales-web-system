import { apiFetch } from '@/lib/api/client';

export type FraudAlert = {
  id: string;
  eventType: string;
  severity: string;
  status: string;
  metadata: Record<string, unknown>;
  createdAt: string;
};

export async function fetchFraudAlerts(limit = 20): Promise<FraudAlert[]> {
  return apiFetch<FraudAlert[]>(`/fraud/alerts?limit=${String(limit)}`);
}

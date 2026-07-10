import { apiFetch, apiPost } from '@/lib/api/client';
import type { HealthHistoryPoint, HealthMatrixResponse } from '@/lib/api/types';

export async function fetchHealthMatrix(): Promise<HealthMatrixResponse> {
  return apiFetch<HealthMatrixResponse>('/platform/health/matrix');
}

export async function fetchHealthHistory(params: {
  probe: string;
  since: string;
}): Promise<{ points: HealthHistoryPoint[] }> {
  const search = new URLSearchParams({ probe: params.probe, since: params.since });
  return apiFetch(`/platform/health/history?${search.toString()}`);
}

export async function scheduleMaintenance(body: {
  tenantId?: string;
  message: string;
  startsAt: string;
  endsAt: string;
}) {
  return apiPost('/platform/maintenance', body);
}

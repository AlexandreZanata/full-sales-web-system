import { apiFetch, apiPost } from '@/lib/api/client';
import type {
  GenerateReportRequest,
  PaginatedResponse,
  Report,
  VerifyReportResponse,
} from '@/lib/api/types';

export type ReportsListParams = {
  page: number;
  pageSize: number;
};

function buildReportsQuery(params: ReportsListParams): URLSearchParams {
  return new URLSearchParams({
    page: String(params.page),
    pageSize: String(params.pageSize),
  });
}

export async function fetchReports(params: ReportsListParams): Promise<PaginatedResponse<Report>> {
  const query = buildReportsQuery(params);
  return apiFetch<PaginatedResponse<Report>>(`/reports?${query}`);
}

export async function fetchReport(id: string): Promise<Report> {
  return apiFetch<Report>(`/reports/${id}`);
}

export async function generateReport(body: GenerateReportRequest): Promise<Report> {
  return apiPost<Report>('/reports', body);
}

export async function verifyReport(id: string): Promise<VerifyReportResponse> {
  return apiFetch<VerifyReportResponse>(`/reports/${id}/verify`, { skipAuth: true });
}

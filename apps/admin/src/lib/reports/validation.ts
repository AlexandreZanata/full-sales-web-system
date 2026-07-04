import type { GenerateReportRequest } from '@/lib/api/types';
import { dateFilterToIso } from '@/lib/api/orders';
import { REPORT_TYPES } from '@/lib/reports/constants';

export type GenerateReportFormValues = {
  reportType: GenerateReportRequest['reportType'] | '';
  periodStart: string;
  periodEnd: string;
  driverId: string;
  commerceId: string;
};

export type GenerateReportFormErrors = Partial<
  Record<'reportType' | 'periodStart' | 'periodEnd' | 'driverId' | 'commerceId', string>
>;

export function validateGenerateReportForm(
  values: GenerateReportFormValues,
): GenerateReportFormErrors {
  const errors: GenerateReportFormErrors = {};

  if (!values.reportType || !REPORT_TYPES.includes(values.reportType)) {
    errors.reportType = 'Select a report type';
  }

  if (!values.periodStart) {
    errors.periodStart = 'Enter a start date';
  }

  if (!values.periodEnd) {
    errors.periodEnd = 'Enter an end date';
  }

  if (values.periodStart && values.periodEnd && values.periodStart > values.periodEnd) {
    errors.periodEnd = 'End date must be on or after start date';
  }

  if (!values.driverId) {
    errors.driverId = 'Select a driver';
  }

  return errors;
}

export function hasFormErrors(errors: GenerateReportFormErrors): boolean {
  return Object.keys(errors).length > 0;
}

export function toGenerateReportPayload(values: GenerateReportFormValues): GenerateReportRequest {
  const payload: GenerateReportRequest = {
    reportType: values.reportType as GenerateReportRequest['reportType'],
    periodStart: dateFilterToIso(values.periodStart, 'start'),
    periodEnd: dateFilterToIso(values.periodEnd, 'end'),
    driverId: values.driverId,
  };

  if (values.commerceId) {
    payload.commerceId = values.commerceId;
  }

  return payload;
}

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
    errors.reportType = 'forms.validation.selectReportType';
  }

  if (!values.periodStart) {
    errors.periodStart = 'forms.validation.periodStartRequired';
  }

  if (!values.periodEnd) {
    errors.periodEnd = 'forms.validation.periodEndRequired';
  }

  if (values.periodStart && values.periodEnd && values.periodStart > values.periodEnd) {
    errors.periodEnd = 'forms.validation.periodEndBeforeStart';
  }

  if (!values.driverId) {
    errors.driverId = 'forms.validation.selectDriver';
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

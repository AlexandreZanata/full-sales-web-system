/**
 * Contract: Phase 35 generate report form — type, period, driver validation.
 */
import { describe, expect, it } from 'vitest';

import { dateFilterToIso } from '@/lib/api/orders';
import {
  hasFormErrors,
  toGenerateReportPayload,
  validateGenerateReportForm,
  type GenerateReportFormValues,
} from '@/lib/reports/validation';

const validForm: GenerateReportFormValues = {
  reportType: 'DailyDriver',
  periodStart: '2026-01-01',
  periodEnd: '2026-01-31',
  driverId: '550e8400-e29b-41d4-a716-446655440000',
  commerceId: '',
};

describe('validateGenerateReportForm — Phase 35 contract', () => {
  it('given_valid_form_when_validate_then_no_errors', () => {
    expect(hasFormErrors(validateGenerateReportForm(validForm))).toBe(false);
  });

  it('given_missing_report_type_when_validate_then_type_error', () => {
    const errors = validateGenerateReportForm({ ...validForm, reportType: '' });
    expect(errors.reportType).toBe('Select a report type');
  });

  it('given_end_before_start_when_validate_then_period_error', () => {
    const errors = validateGenerateReportForm({
      ...validForm,
      periodStart: '2026-02-01',
      periodEnd: '2026-01-01',
    });
    expect(errors.periodEnd).toBe('End date must be on or after start date');
  });

  it('given_missing_driver_when_validate_then_driver_error', () => {
    const errors = validateGenerateReportForm({ ...validForm, driverId: '' });
    expect(errors.driverId).toBe('Select a driver');
  });
});

describe('toGenerateReportPayload — API contract', () => {
  it('given_valid_form_when_to_payload_then_iso_period_and_driver', () => {
    const payload = toGenerateReportPayload(validForm);
    expect(payload.reportType).toBe('DailyDriver');
    expect(payload.periodStart).toBe(dateFilterToIso('2026-01-01', 'start'));
    expect(payload.periodEnd).toBe(dateFilterToIso('2026-01-31', 'end'));
    expect(payload.driverId).toBe(validForm.driverId);
    expect(payload.commerceId).toBeUndefined();
  });

  it('given_commerce_filter_when_to_payload_then_includes_commerce_id', () => {
    const payload = toGenerateReportPayload({
      ...validForm,
      commerceId: '660e8400-e29b-41d4-a716-446655440001',
    });
    expect(payload.commerceId).toBe('660e8400-e29b-41d4-a716-446655440001');
  });
});

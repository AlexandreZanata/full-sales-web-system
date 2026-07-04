import { useQuery } from '@tanstack/react-query';
import { useState, type SubmitEvent } from 'react';

import { Button } from '@/components/ui/Button';
import { Card } from '@/components/ui/Card';
import { Input } from '@/components/ui/Input';
import { Select } from '@/components/ui/Select';
import { useToast } from '@/hooks/useToast';
import { ApiError } from '@/lib/api/client';
import { fetchCommercesForPicker } from '@/lib/api/commerces';
import type { GenerateReportRequest, Report } from '@/lib/api/types';
import { fetchDriversForPicker } from '@/lib/api/users';
import { REPORT_TYPE_LABELS, REPORT_TYPES } from '@/lib/reports/constants';
import { reportActionErrorMessage } from '@/lib/reports/reportActionErrors';
import {
  hasFormErrors,
  toGenerateReportPayload,
  validateGenerateReportForm,
  type GenerateReportFormValues,
} from '@/lib/reports/validation';

const emptyForm: GenerateReportFormValues = {
  reportType: '',
  periodStart: '',
  periodEnd: '',
  driverId: '',
  commerceId: '',
};

type CreateReportFormProps = {
  onSubmit: (body: GenerateReportRequest) => Promise<Report>;
  onSuccess: (report: Report) => void;
};

export function CreateReportForm({ onSubmit, onSuccess }: CreateReportFormProps) {
  const toast = useToast();
  const [values, setValues] = useState<GenerateReportFormValues>(emptyForm);
  const [errors, setErrors] = useState<ReturnType<typeof validateGenerateReportForm>>({});
  const [submitting, setSubmitting] = useState(false);

  const drivers = useQuery({
    queryKey: ['users', 'drivers', 'picker'],
    queryFn: fetchDriversForPicker,
  });

  const commerces = useQuery({
    queryKey: ['commerces', 'picker'],
    queryFn: fetchCommercesForPicker,
  });

  async function handleSubmit(event: SubmitEvent<HTMLFormElement>) {
    event.preventDefault();
    const nextErrors = validateGenerateReportForm(values);
    setErrors(nextErrors);
    if (hasFormErrors(nextErrors)) {
      return;
    }

    setSubmitting(true);
    try {
      const report = await onSubmit(toGenerateReportPayload(values));
      toast.success('Report generated');
      onSuccess(report);
    } catch (error) {
      const message =
        error instanceof ApiError
          ? reportActionErrorMessage(error.code)
          : 'Unable to generate report';
      toast.error(message);
    } finally {
      setSubmitting(false);
    }
  }

  return (
    <Card>
      <form className="space-y-4" onSubmit={(event) => void handleSubmit(event)}>
        <Select
          label="Report type"
          value={values.reportType}
          error={errors.reportType}
          onChange={(event) => {
            setValues((current) => ({
              ...current,
              reportType: event.target.value as GenerateReportFormValues['reportType'],
            }));
          }}
        >
          <option value="">Select report type</option>
          {REPORT_TYPES.map((type) => (
            <option key={type} value={type}>
              {REPORT_TYPE_LABELS[type]}
            </option>
          ))}
        </Select>

        <div className="grid gap-4 sm:grid-cols-2">
          <Input
            label="Period start"
            type="date"
            value={values.periodStart}
            error={errors.periodStart}
            onChange={(event) => {
              setValues((current) => ({ ...current, periodStart: event.target.value }));
            }}
          />
          <Input
            label="Period end"
            type="date"
            value={values.periodEnd}
            error={errors.periodEnd}
            onChange={(event) => {
              setValues((current) => ({ ...current, periodEnd: event.target.value }));
            }}
          />
        </div>

        <Select
          label="Driver"
          value={values.driverId}
          error={errors.driverId}
          disabled={drivers.isLoading}
          onChange={(event) => {
            setValues((current) => ({ ...current, driverId: event.target.value }));
          }}
        >
          <option value="">Select driver</option>
          {(drivers.data ?? []).map((driver) => (
            <option key={driver.id} value={driver.id}>
              {driver.name}
            </option>
          ))}
        </Select>

        <Select
          label="Commerce (optional filter)"
          value={values.commerceId}
          error={errors.commerceId}
          disabled={commerces.isLoading}
          onChange={(event) => {
            setValues((current) => ({ ...current, commerceId: event.target.value }));
          }}
        >
          <option value="">All commerces</option>
          {(commerces.data ?? []).map((commerce) => (
            <option key={commerce.id} value={commerce.id}>
              {commerce.tradeName || commerce.legalName}
            </option>
          ))}
        </Select>

        <p className="text-xs text-muted-foreground">
          Empty periods produce a signed report with zero totals (ADR-003).
        </p>

        <Button type="submit" disabled={submitting}>
          {submitting ? 'Generating…' : 'Generate report'}
        </Button>
      </form>
    </Card>
  );
}

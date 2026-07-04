import { useState, type SubmitEvent } from 'react';

import { Button } from '@/components/ui/Button';
import { Input } from '@/components/ui/Input';
import { useToast } from '@/hooks/useToast';
import { ApiError } from '@/lib/api/client';
import { upsertSellerProfile } from '@/lib/api/users';
import { formatApiErrorMessage } from '@/lib/utils';

type SellerProfileTabProps = {
  userId: string;
};

type SellerFormValues = {
  operatingRegion: string;
  monthlyTargetAmount: string;
};

const emptyForm: SellerFormValues = {
  operatingRegion: '',
  monthlyTargetAmount: '',
};

export function SellerProfileTab({ userId }: SellerProfileTabProps) {
  const toast = useToast();
  const [values, setValues] = useState<SellerFormValues>(emptyForm);
  const [submitting, setSubmitting] = useState(false);

  async function handleSubmit(event: SubmitEvent<HTMLFormElement>) {
    event.preventDefault();
    const body: { operatingRegion?: string; monthlyTargetAmount?: number } = {};
    const region = values.operatingRegion.trim();
    const target = values.monthlyTargetAmount.trim();

    if (region) body.operatingRegion = region;
    if (target) body.monthlyTargetAmount = Number(target);

    setSubmitting(true);
    try {
      await upsertSellerProfile(userId, body);
      toast.success('Seller profile saved');
    } catch (error) {
      const message =
        error instanceof ApiError
          ? formatApiErrorMessage(error.message, error.code)
          : 'Unable to save seller profile';
      toast.error(message);
    } finally {
      setSubmitting(false);
    }
  }

  return (
    <form className="space-y-4" onSubmit={(event) => void handleSubmit(event)}>
      <Input
        label="Operating region"
        name="operatingRegion"
        value={values.operatingRegion}
        onChange={(event) => {
          setValues((current) => ({ ...current, operatingRegion: event.target.value }));
        }}
      />
      <Input
        label="Monthly target amount"
        name="monthlyTargetAmount"
        type="number"
        min="0"
        step="1"
        value={values.monthlyTargetAmount}
        onChange={(event) => {
          setValues((current) => ({ ...current, monthlyTargetAmount: event.target.value }));
        }}
      />
      <Button type="submit" disabled={submitting}>
        {submitting ? 'Saving…' : 'Save seller profile'}
      </Button>
    </form>
  );
}

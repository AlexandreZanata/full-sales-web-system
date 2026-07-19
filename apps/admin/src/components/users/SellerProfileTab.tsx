import { useQuery } from '@tanstack/react-query';
import { useEffect, useState, type SubmitEvent } from 'react';

import { Button } from '@/components/ui/Button';
import { Input } from '@/components/ui/Input';
import { LoadingSpinner } from '@/components/ui/LoadingSpinner';
import { useToast } from '@/hooks/useToast';
import { ApiError } from '@/lib/api/client';
import { fetchSellerProfile, upsertSellerProfile } from '@/lib/api/sellerProfile';
import { useI18n } from '@/lib/i18n/context';
import { formatApiErrorMessage } from '@/lib/utils';

type SellerProfileTabProps = {
  userId: string;
};

type SellerFormValues = {
  operatingRegion: string;
  monthlyTargetAmount: string;
  contactPhone: string;
  publicCode: string;
  shareLinkActive: boolean;
};

const emptyForm: SellerFormValues = {
  operatingRegion: '',
  monthlyTargetAmount: '',
  contactPhone: '',
  publicCode: '',
  shareLinkActive: true,
};

export function SellerProfileTab({ userId }: SellerProfileTabProps) {
  const { t } = useI18n();
  const toast = useToast();
  const [values, setValues] = useState<SellerFormValues>(emptyForm);
  const [submitting, setSubmitting] = useState(false);

  const profile = useQuery({
    queryKey: ['users', userId, 'seller-profile'],
    queryFn: () => fetchSellerProfile(userId),
  });

  useEffect(() => {
    if (!profile.data) {
      return;
    }
    setValues({
      operatingRegion: profile.data.operatingRegion ?? '',
      monthlyTargetAmount:
        profile.data.monthlyTargetAmount != null ? String(profile.data.monthlyTargetAmount) : '',
      contactPhone: profile.data.contactPhone ?? '',
      publicCode: profile.data.publicCode ?? '',
      shareLinkActive: profile.data.shareLinkActive ?? true,
    });
  }, [profile.data]);

  async function handleSubmit(event: SubmitEvent<HTMLFormElement>) {
    event.preventDefault();
    const target = values.monthlyTargetAmount.trim();
    setSubmitting(true);
    try {
      await upsertSellerProfile(userId, {
        operatingRegion: values.operatingRegion.trim() || undefined,
        monthlyTargetAmount: target ? Number(target) : undefined,
        contactPhone: values.contactPhone.trim(),
        publicCode: values.publicCode.trim() || undefined,
        shareLinkActive: values.shareLinkActive,
      });
      await profile.refetch();
      toast.success(t('users.toast.sellerProfileSaved'));
    } catch (error) {
      const message =
        error instanceof ApiError
          ? formatApiErrorMessage(error.message, error.code)
          : t('errors.actionFailed');
      toast.error(message);
    } finally {
      setSubmitting(false);
    }
  }

  if (profile.isLoading) {
    return (
      <div className="flex justify-center py-8">
        <LoadingSpinner />
      </div>
    );
  }

  const shareHint = t('users.sellerProfile.sharePathHint').replace(
    '{code}',
    values.publicCode.trim() || '…',
  );

  return (
    <form className="space-y-4" onSubmit={(event) => void handleSubmit(event)}>
      <Input
        label={t('forms.fields.operatingRegion')}
        name="operatingRegion"
        value={values.operatingRegion}
        onChange={(event) => {
          setValues((current) => ({ ...current, operatingRegion: event.target.value }));
        }}
      />
      <Input
        label={t('forms.fields.monthlyTarget')}
        name="monthlyTargetAmount"
        type="number"
        min="0"
        step="1"
        value={values.monthlyTargetAmount}
        onChange={(event) => {
          setValues((current) => ({ ...current, monthlyTargetAmount: event.target.value }));
        }}
      />
      <Input
        label={t('forms.fields.contactPhone')}
        name="contactPhone"
        inputMode="tel"
        placeholder="11999998888"
        value={values.contactPhone}
        onChange={(event) => {
          setValues((current) => ({ ...current, contactPhone: event.target.value }));
        }}
      />
      <Input
        label={t('forms.fields.publicCode')}
        name="publicCode"
        value={values.publicCode}
        onChange={(event) => {
          setValues((current) => ({ ...current, publicCode: event.target.value }));
        }}
      />
      <p className="text-xs text-muted-foreground">{shareHint}</p>
      <label className="flex items-center gap-2 text-sm">
        <input
          type="checkbox"
          checked={values.shareLinkActive}
          onChange={(event) => {
            setValues((current) => ({ ...current, shareLinkActive: event.target.checked }));
          }}
        />
        {t('forms.fields.shareLinkActive')}
      </label>
      <Button type="submit" disabled={submitting}>
        {submitting ? t('users.sellerProfile.saving') : t('users.sellerProfile.save')}
      </Button>
    </form>
  );
}

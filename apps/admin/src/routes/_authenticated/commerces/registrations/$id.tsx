import { Link, createFileRoute } from '@tanstack/react-router';
import { useQuery, useQueryClient } from '@tanstack/react-query';
import { useState } from 'react';

import { Button } from '@/components/ui/Button';
import { Card } from '@/components/ui/Card';
import { Input } from '@/components/ui/Input';
import { LoadingSpinner } from '@/components/ui/LoadingSpinner';
import { PageBackLink } from '@/components/ui/PageBackLink';
import { PageHeader } from '@/components/ui/PageHeader';
import { useToast } from '@/hooks/useToast';
import {
  approveCommerceRegistration,
  fetchCommerceRegistration,
  rejectCommerceRegistration,
} from '@/lib/api/commerceRegistrations';
import { formatCnpj } from '@/lib/commerces/cnpj';
import { useI18n } from '@/lib/i18n/context';

export const Route = createFileRoute('/_authenticated/commerces/registrations/$id')({
  component: CommerceRegistrationDetailPage,
});

function CommerceRegistrationDetailPage() {
  const { id } = Route.useParams();
  const { t } = useI18n();
  const toast = useToast();
  const queryClient = useQueryClient();
  const [rejectOpen, setRejectOpen] = useState(false);
  const [rejectReason, setRejectReason] = useState('');
  const [acting, setActing] = useState(false);

  const registration = useQuery({
    queryKey: ['commerce-registrations', id],
    queryFn: () => fetchCommerceRegistration(id),
  });

  async function handleApprove() {
    setActing(true);
    try {
      await approveCommerceRegistration(id);
      await queryClient.invalidateQueries({ queryKey: ['commerce-registrations'] });
      await queryClient.invalidateQueries({ queryKey: ['commerces'] });
      toast.success(t('commerces.registrations.toast.approved'));
    } catch {
      toast.error(t('errors.actionFailed'));
    } finally {
      setActing(false);
    }
  }

  async function handleReject() {
    if (!rejectReason.trim()) {
      return;
    }
    setActing(true);
    try {
      await rejectCommerceRegistration(id, { reason: rejectReason.trim() });
      await queryClient.invalidateQueries({ queryKey: ['commerce-registrations'] });
      toast.success(t('commerces.registrations.toast.rejected'));
      setRejectOpen(false);
    } catch {
      toast.error(t('errors.actionFailed'));
    } finally {
      setActing(false);
    }
  }

  if (registration.isLoading) {
    return (
      <div className="flex justify-center py-16">
        <LoadingSpinner />
      </div>
    );
  }

  const data = registration.data;
  if (!data) {
    return (
      <PageHeader
        title={t('commerces.registrations.notFound')}
        back={
          <PageBackLink
            label={t('commerces.registrations.backToQueue')}
            to="/commerces/registrations"
          />
        }
      />
    );
  }

  const pending = data.registrationStatus === 'PendingReview';

  return (
    <div className="space-y-6">
      <PageHeader
        title={data.tradeName}
        description={t(`commerces.registrations.status.${data.registrationStatus}`)}
        back={
          <PageBackLink
            label={t('commerces.registrations.backToQueue')}
            to="/commerces/registrations"
          />
        }
        actions={
          pending ? (
            <div className="flex gap-2">
              <Button
                disabled={acting}
                onClick={() => {
                  void handleApprove();
                }}
              >
                {t('commerces.registrations.approve')}
              </Button>
              <Button
                disabled={acting}
                variant="secondary"
                onClick={() => {
                  setRejectOpen(true);
                }}
              >
                {t('commerces.registrations.reject')}
              </Button>
            </div>
          ) : data.registrationStatus === 'Active' ? (
            <Link to="/commerces/$id" params={{ id: data.id }}>
              <Button variant="secondary">{t('commerces.registrations.openCommerce')}</Button>
            </Link>
          ) : null
        }
      />

      <Card className="space-y-3 p-4">
        <p>
          <span className="font-medium">{t('commerces.registrations.fields.cnpj')}: </span>
          {formatCnpj(data.cnpj)}
        </p>
        <p>
          <span className="font-medium">{t('commerces.registrations.fields.legalName')}: </span>
          {data.legalName}
        </p>
        {data.registrationMode ? (
          <p>
            <span className="font-medium">{t('commerces.registrations.fields.mode')}: </span>
            {t(`commerces.registrations.mode.${data.registrationMode}`)}
          </p>
        ) : null}
        {data.rejectionReason ? (
          <p>
            <span className="font-medium">
              {t('commerces.registrations.fields.rejectionReason')}:{' '}
            </span>
            {data.rejectionReason}
          </p>
        ) : null}
      </Card>

      {rejectOpen ? (
        <Card className="space-y-4 p-4">
          <Input
            label={t('commerces.registrations.rejectDialog.reason')}
            value={rejectReason}
            onChange={(event) => {
              setRejectReason(event.target.value);
            }}
          />
          <div className="flex gap-2">
            <Button
              disabled={acting || !rejectReason.trim()}
              onClick={() => {
                void handleReject();
              }}
            >
              {t('commerces.registrations.reject')}
            </Button>
            <Button
              variant="secondary"
              onClick={() => {
                setRejectOpen(false);
              }}
            >
              {t('common.cancel')}
            </Button>
          </div>
        </Card>
      ) : null}
    </div>
  );
}

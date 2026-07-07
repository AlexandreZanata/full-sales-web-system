import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';
import { Link, createFileRoute } from '@tanstack/react-router';
import { useId, useState } from 'react';

import { DeliveryStatusBadge } from '@/components/deliveries/DeliveryStatusBadge';
import { Button } from '@/components/ui/Button';
import { Card } from '@/components/ui/Card';
import { EmptyState } from '@/components/ui/EmptyState';
import { LoadingSpinner } from '@/components/ui/LoadingSpinner';
import { confirmDelivery, fetchDelivery, startDeliveryTransit } from '@/lib/api/deliveries';
import { uploadDeliveryProof } from '@/lib/api/uploads';
import { ApiError } from '@/lib/api/client';
import { useI18n } from '@/lib/i18n/context';
import { deliveryActionErrorMessage } from '@/lib/deliveries/constants';

export const Route = createFileRoute('/_authenticated/deliveries/$id')({
  component: DeliveryDetailPage,
});

function DeliveryDetailPage() {
  const { id } = Route.useParams();
  const { t } = useI18n();
  const queryClient = useQueryClient();
  const proofInputId = useId();
  const [error, setError] = useState<string | null>(null);

  const deliveryQuery = useQuery({
    queryKey: ['deliveries', id],
    queryFn: () => fetchDelivery(id),
  });

  const invalidate = async () => {
    await queryClient.invalidateQueries({ queryKey: ['deliveries'] });
    await deliveryQuery.refetch();
  };

  const transitMutation = useMutation({
    mutationFn: () => startDeliveryTransit(id),
    onSuccess: () => void invalidate(),
    onError: (err) => {
      setError(
        err instanceof ApiError ? deliveryActionErrorMessage(err.code) : t('common.loadFailed'),
      );
    },
  });

  const confirmMutation = useMutation({
    mutationFn: async (file: File) => {
      const proof = await uploadDeliveryProof(file, id);
      const items = (deliveryQuery.data?.orderItems ?? []).map((item) => ({
        orderItemId: item.id,
        quantityDelivered: item.quantity,
      }));
      return confirmDelivery(id, { proofFileId: proof.id, items });
    },
    onSuccess: () => void invalidate(),
    onError: (err) => {
      setError(
        err instanceof ApiError ? deliveryActionErrorMessage(err.code) : t('common.loadFailed'),
      );
    },
  });

  if (deliveryQuery.isLoading) {
    return <LoadingSpinner className="py-16" />;
  }

  if (deliveryQuery.isError || !deliveryQuery.data) {
    return <EmptyState title={t('common.loadFailed')} />;
  }

  const delivery = deliveryQuery.data;
  const isWaiting = delivery.status === 'Waiting';
  const isInTransit = delivery.status === 'InTransit';

  return (
    <div className="space-y-4 pb-24">
      <Link to="/deliveries" className="text-sm text-muted-foreground hover:text-foreground">
        ← {t('deliveries.title')}
      </Link>
      <div className="flex items-center justify-between gap-3">
        <h1 className="text-2xl font-semibold">{t('deliveries.detail')}</h1>
        <DeliveryStatusBadge status={delivery.status} />
      </div>

      <Card className="space-y-3 text-sm">
        <p>
          {t('deliveries.orderId')}: <span className="font-mono text-xs">{delivery.orderId}</span>
        </p>
        {delivery.orderItems?.length ? (
          <ul className="divide-y divide-hairline border-y border-hairline">
            {delivery.orderItems.map((item) => (
              <li key={item.id} className="flex justify-between py-2">
                <span className="font-mono text-xs">{item.productId.slice(0, 8)}</span>
                <span>{item.quantity}×</span>
              </li>
            ))}
          </ul>
        ) : null}
        {error ? <p className="text-destructive">{error}</p> : null}
      </Card>

      {isWaiting ? (
        <Button
          className="w-full"
          disabled={transitMutation.isPending}
          onClick={() => {
            setError(null);
            transitMutation.mutate();
          }}
        >
          {transitMutation.isPending ? t('common.working') : t('deliveries.startTransit')}
        </Button>
      ) : null}

      {isInTransit ? (
        <div className="space-y-3">
          <label
            htmlFor={proofInputId}
            className="text-xs font-semibold uppercase tracking-[0.12em] text-muted-foreground"
          >
            {t('deliveries.proofPhoto')}
          </label>
          <input
            id={proofInputId}
            type="file"
            accept="image/*"
            className="sr-only"
            onChange={(event) => {
              const file = event.target.files?.[0];
              if (!file) return;
              setError(null);
              confirmMutation.mutate(file);
            }}
          />
          <Button
            className="w-full"
            disabled={confirmMutation.isPending}
            onClick={() => document.getElementById(proofInputId)?.click()}
          >
            {confirmMutation.isPending ? t('common.working') : t('deliveries.confirm')}
          </Button>
        </div>
      ) : null}
    </div>
  );
}

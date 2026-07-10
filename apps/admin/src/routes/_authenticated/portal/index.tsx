import { createFileRoute } from '@tanstack/react-router';
import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';
import { useState } from 'react';

import { Button } from '@/components/ui/Button';
import { Card } from '@/components/ui/Card';
import { Input } from '@/components/ui/Input';
import { PageHeader } from '@/components/ui/PageHeader';
import { Select } from '@/components/ui/Select';
import { useToast } from '@/hooks/useToast';
import {
  createPortalBanner,
  createPortalPromotion,
  deletePortalBanner,
  deletePortalPromotion,
  fetchPortalBanners,
  fetchPortalPromotions,
} from '@/lib/api/portalContent';
import { useI18n } from '@/lib/i18n/context';

export const Route = createFileRoute('/_authenticated/portal/')({
  component: PortalContentPage,
});

function PortalContentPage() {
  const { t } = useI18n();
  const toast = useToast();
  const queryClient = useQueryClient();
  const banners = useQuery({ queryKey: ['portal-banners'], queryFn: fetchPortalBanners });
  const promotions = useQuery({ queryKey: ['portal-promotions'], queryFn: fetchPortalPromotions });
  const [bannerFileId, setBannerFileId] = useState('');
  const [promoHeadline, setPromoHeadline] = useState('');
  const [promoDiscount, setPromoDiscount] = useState('');
  const [promoBackground, setPromoBackground] = useState<'yellow' | 'green'>('yellow');
  const [promoCategorySlug, setPromoCategorySlug] = useState('bebidas');

  const createBanner = useMutation({
    mutationFn: () =>
      createPortalBanner({
        imageFileId: bannerFileId.trim(),
        placement: 'hero',
        altText: 'Hero banner',
        sortOrder: (banners.data?.length ?? 0) + 1,
      }),
    onSuccess: async () => {
      await queryClient.invalidateQueries({ queryKey: ['portal-banners'] });
      setBannerFileId('');
      toast.success(t('portal.toast.bannerCreated'));
    },
    onError: () => toast.error(t('errors.actionFailed')),
  });

  const createPromotion = useMutation({
    mutationFn: () =>
      createPortalPromotion({
        headline: promoHeadline.trim(),
        discountText: promoDiscount.trim(),
        background: promoBackground,
        categorySlug: promoCategorySlug.trim() || undefined,
        sortOrder: (promotions.data?.length ?? 0) + 1,
      }),
    onSuccess: async () => {
      await queryClient.invalidateQueries({ queryKey: ['portal-promotions'] });
      setPromoHeadline('');
      setPromoDiscount('');
      toast.success(t('portal.toast.promotionCreated'));
    },
    onError: () => toast.error(t('errors.actionFailed')),
  });

  const removeBanner = useMutation({
    mutationFn: deletePortalBanner,
    onSuccess: async () => {
      await queryClient.invalidateQueries({ queryKey: ['portal-banners'] });
      toast.success(t('portal.toast.bannerDeleted'));
    },
    onError: () => toast.error(t('errors.actionFailed')),
  });

  const removePromotion = useMutation({
    mutationFn: deletePortalPromotion,
    onSuccess: async () => {
      await queryClient.invalidateQueries({ queryKey: ['portal-promotions'] });
      toast.success(t('portal.toast.promotionDeleted'));
    },
    onError: () => toast.error(t('errors.actionFailed')),
  });

  return (
    <div className="space-y-6">
      <PageHeader title={t('portal.title')} description={t('portal.description')} />
      <Card>
        <h2 className="mb-4 text-lg font-semibold">{t('portal.banners')}</h2>
        <div className="mb-4 flex flex-wrap gap-2">
          <Input
            label={t('portal.bannerImageFileId')}
            value={bannerFileId}
            onChange={(event) => setBannerFileId(event.target.value)}
          />
          <Button
            className="self-end"
            disabled={!bannerFileId.trim() || createBanner.isPending}
            onClick={() => createBanner.mutate()}
          >
            {t('portal.addBanner')}
          </Button>
        </div>
        <ul className="space-y-2 text-sm">
          {(banners.data ?? []).map((banner) => (
            <li key={banner.id} className="flex items-center justify-between gap-2 rounded border p-2">
              <span>
                {banner.placement} · {banner.altText ?? banner.id}
              </span>
              <Button
                variant="ghost"
                onClick={() => removeBanner.mutate(banner.id)}
                disabled={removeBanner.isPending}
              >
                {t('common.remove')}
              </Button>
            </li>
          ))}
        </ul>
      </Card>
      <Card>
        <h2 className="mb-4 text-lg font-semibold">{t('portal.promotions')}</h2>
        <div className="mb-4 grid gap-3 sm:grid-cols-2">
          <Input label={t('portal.promoHeadline')} value={promoHeadline} onChange={(e) => setPromoHeadline(e.target.value)} />
          <Input label={t('portal.promoDiscount')} value={promoDiscount} onChange={(e) => setPromoDiscount(e.target.value)} />
          <Select
            label={t('portal.promoBackground')}
            value={promoBackground}
            onChange={(event) => setPromoBackground(event.target.value as 'yellow' | 'green')}
          >
            <option value="yellow">yellow</option>
            <option value="green">green</option>
          </Select>
          <Input label={t('portal.promoCategorySlug')} value={promoCategorySlug} onChange={(e) => setPromoCategorySlug(e.target.value)} />
        </div>
        <Button
          className="mb-4"
          disabled={!promoHeadline.trim() || !promoDiscount.trim() || createPromotion.isPending}
          onClick={() => createPromotion.mutate()}
        >
          {t('portal.addPromotion')}
        </Button>
        <ul className="space-y-2 text-sm">
          {(promotions.data ?? []).map((promotion) => (
            <li key={promotion.id} className="flex items-center justify-between gap-2 rounded border p-2">
              <span>
                {promotion.headline} · {promotion.discountText}
              </span>
              <Button
                variant="ghost"
                onClick={() => removePromotion.mutate(promotion.id)}
                disabled={removePromotion.isPending}
              >
                {t('common.remove')}
              </Button>
            </li>
          ))}
        </ul>
      </Card>
    </div>
  );
}

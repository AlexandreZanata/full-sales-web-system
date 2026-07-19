import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';
import { useState } from 'react';

import { PortalBannerThumb } from '@/components/portal/PortalBannerThumb';
import { Button } from '@/components/ui/Button';
import { Card } from '@/components/ui/Card';
import { Input } from '@/components/ui/Input';
import { FileUploadField } from '@/components/uploads/FileUploadField';
import { useToast } from '@/hooks/useToast';
import {
  createPortalBanner,
  deletePortalBanner,
  fetchPortalBanners,
} from '@/lib/api/portalContent';
import { useI18n } from '@/lib/i18n/context';

function newDraftBannerId(): string {
  return crypto.randomUUID();
}

export function PortalBannersSection() {
  const { t } = useI18n();
  const toast = useToast();
  const queryClient = useQueryClient();
  const banners = useQuery({ queryKey: ['portal-banners'], queryFn: fetchPortalBanners });
  const [draftEntityId, setDraftEntityId] = useState(newDraftBannerId);
  const [bannerFileId, setBannerFileId] = useState('');
  const [bannerImageUrl, setBannerImageUrl] = useState('');
  const [bannerLinkUrl, setBannerLinkUrl] = useState('');

  const createBanner = useMutation({
    mutationFn: () =>
      createPortalBanner({
        imageFileId: bannerFileId.trim() || undefined,
        imageUrl: bannerImageUrl.trim() || undefined,
        linkUrl: bannerLinkUrl.trim() || undefined,
        placement: 'hero',
        altText: 'Hero banner',
        sortOrder: (banners.data?.length ?? 0) + 1,
      }),
    onSuccess: async () => {
      await queryClient.invalidateQueries({ queryKey: ['portal-banners'] });
      setBannerFileId('');
      setBannerImageUrl('');
      setBannerLinkUrl('');
      setDraftEntityId(newDraftBannerId());
      toast.success(t('portal.toast.bannerCreated'));
    },
    onError: () => {
      toast.error(t('errors.actionFailed'));
    },
  });

  const removeBanner = useMutation({
    mutationFn: deletePortalBanner,
    onSuccess: async () => {
      await queryClient.invalidateQueries({ queryKey: ['portal-banners'] });
      toast.success(t('portal.toast.bannerDeleted'));
    },
    onError: () => {
      toast.error(t('errors.actionFailed'));
    },
  });

  const canSubmit = Boolean(bannerFileId.trim() || bannerImageUrl.trim());

  return (
    <Card>
      <h2 className="mb-4 text-lg font-semibold">{t('portal.banners')}</h2>
      <div className="mb-6 space-y-4">
        <FileUploadField
          label={t('portal.bannerImage')}
          fileId={bannerFileId}
          onChange={setBannerFileId}
          entityType="PortalBanner"
          entityId={draftEntityId}
          previewMode="authenticated"
        />
        <Input
          label={t('portal.bannerImageUrl')}
          value={bannerImageUrl}
          placeholder="https://"
          onChange={(event) => {
            setBannerImageUrl(event.target.value);
          }}
        />
        <Input
          label={t('portal.bannerLinkUrl')}
          value={bannerLinkUrl}
          placeholder="https://"
          onChange={(event) => {
            setBannerLinkUrl(event.target.value);
          }}
        />
        {(bannerFileId || bannerImageUrl.trim()) && (
          <div className="max-w-xl">
            <p className="mb-2 text-xs font-semibold uppercase tracking-[0.12em] text-muted-foreground">
              {t('portal.bannerPreview')}
            </p>
            <PortalBannerThumb
              imageFileId={bannerFileId || undefined}
              imageUrl={bannerImageUrl.trim() || undefined}
              alt={t('portal.bannerPreview')}
            />
          </div>
        )}
        <Button
          disabled={!canSubmit || createBanner.isPending}
          onClick={() => {
            createBanner.mutate();
          }}
        >
          {t('portal.addBanner')}
        </Button>
      </div>

      <ul className="grid gap-4 sm:grid-cols-2">
        {(banners.data ?? []).map((banner) => (
          <li key={banner.id} className="space-y-2 rounded-md border border-hairline p-3">
            <PortalBannerThumb
              imageFileId={banner.imageFileId}
              imageUrl={banner.imageUrl}
              alt={banner.altText ?? t('portal.banners')}
            />
            <div className="flex items-center justify-between gap-2">
              <span className="truncate text-sm text-muted-foreground">
                {banner.placement}
                {banner.altText ? ` · ${banner.altText}` : ''}
              </span>
              <Button
                variant="ghost"
                onClick={() => {
                  removeBanner.mutate(banner.id);
                }}
                disabled={removeBanner.isPending}
              >
                {t('common.remove')}
              </Button>
            </div>
          </li>
        ))}
      </ul>
    </Card>
  );
}

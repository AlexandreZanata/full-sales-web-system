import { useMediaPreviewUrl } from '@/hooks/useMediaPreviewUrl';
import { useI18n } from '@/lib/i18n/context';
import { cn } from '@/lib/utils';

type PortalBannerThumbProps = {
  imageFileId?: string;
  imageUrl?: string;
  alt?: string;
  className?: string;
};

export function PortalBannerThumb({
  imageFileId,
  imageUrl,
  alt = '',
  className,
}: PortalBannerThumbProps) {
  const { t } = useI18n();
  const authPreview = useMediaPreviewUrl(imageFileId);
  const src = imageUrl?.trim() || authPreview;

  if (!src) {
    return (
      <div
        className={cn(
          'flex aspect-[16/7] w-full items-center justify-center rounded-md border border-hairline bg-surface-muted text-xs text-muted-foreground',
          className,
        )}
      >
        {t('uploads.noImage')}
      </div>
    );
  }

  return (
    <img
      src={src}
      alt={alt}
      className={cn(
        'aspect-[16/7] w-full rounded-md border border-hairline object-cover',
        className,
      )}
    />
  );
}

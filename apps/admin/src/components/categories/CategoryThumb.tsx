import { useEffect, useState } from 'react';

import { resolveCategoryThumbUrl, withCatalogImageCacheBust } from '@/lib/api/uploads';
import { useI18n } from '@/lib/i18n/context';

type CategoryThumbProps = {
  name: string;
  imageFileId?: string;
  thumbUrl?: string;
  cacheRevision?: number;
};

export function categoryThumbCacheKey(imageFileId?: string, thumbUrl?: string): string {
  return `${imageFileId ?? ''}|${thumbUrl ?? ''}`;
}

export function CategoryThumb({
  name,
  imageFileId,
  thumbUrl,
  cacheRevision = 0,
}: CategoryThumbProps) {
  const { t } = useI18n();
  const [failed, setFailed] = useState(false);
  const initial = name.trim().charAt(0).toUpperCase();
  const baseSrc = resolveCategoryThumbUrl(imageFileId, thumbUrl);
  const cacheKey = categoryThumbCacheKey(imageFileId, thumbUrl);
  const src = baseSrc ? withCatalogImageCacheBust(baseSrc, cacheKey, cacheRevision) : undefined;

  useEffect(() => {
    setFailed(false);
  }, [cacheKey, cacheRevision]);

  if (!src || failed) {
    return (
      <span
        className="flex size-10 shrink-0 items-center justify-center rounded-md border border-hairline bg-surface-muted text-xs font-semibold text-muted-foreground"
        title={failed ? t('uploads.previewUnavailable') : undefined}
        aria-hidden
      >
        {initial}
      </span>
    );
  }

  return (
    <img
      key={src}
      src={src}
      alt=""
      className="size-10 shrink-0 rounded-md border border-hairline object-cover"
      onError={() => {
        setFailed(true);
      }}
    />
  );
}

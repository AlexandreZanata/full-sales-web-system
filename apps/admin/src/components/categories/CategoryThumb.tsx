import { useState } from 'react';

import { resolveCategoryThumbUrl } from '@/lib/api/uploads';
import { useI18n } from '@/lib/i18n/context';

type CategoryThumbProps = {
  name: string;
  imageFileId?: string;
  thumbUrl?: string;
};

export function CategoryThumb({ name, imageFileId, thumbUrl }: CategoryThumbProps) {
  const { t } = useI18n();
  const [failed, setFailed] = useState(false);
  const initial = name.trim().charAt(0).toUpperCase();
  const src = resolveCategoryThumbUrl(imageFileId, thumbUrl);

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

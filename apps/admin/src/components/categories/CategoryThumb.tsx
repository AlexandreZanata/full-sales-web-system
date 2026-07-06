import { useState } from 'react';

import { resolveCatalogImagePreviewUrl } from '@/lib/api/uploads';
import { useI18n } from '@/lib/i18n/context';

type CategoryThumbProps = {
  name: string;
  imageFileId?: string;
};

export function CategoryThumb({ name, imageFileId }: CategoryThumbProps) {
  const { t } = useI18n();
  const [failed, setFailed] = useState(false);
  const initial = name.trim().charAt(0).toUpperCase();

  if (!imageFileId || failed) {
    return (
      <span
        className="flex size-10 shrink-0 items-center justify-center rounded-md border border-hairline bg-surface-muted text-xs font-semibold text-muted-foreground"
        title={failed ? t('uploads.previewUnavailable') : undefined}
      >
        {initial}
      </span>
    );
  }

  return (
    <img
      src={resolveCatalogImagePreviewUrl(imageFileId)}
      alt=""
      className="size-10 shrink-0 rounded-md border border-hairline object-cover"
      onError={() => {
        setFailed(true);
      }}
    />
  );
}

import { useState } from 'react';

import { resolveProductImagePreviewUrl } from '@/lib/api/uploads';
import { useI18n } from '@/lib/i18n/context';

export function ProductImagePreview({ fileId }: { fileId: string }) {
  const { t } = useI18n();
  const url = resolveProductImagePreviewUrl(fileId);
  const [failed, setFailed] = useState(false);

  if (failed) {
    return (
      <span className="absolute inset-0 flex items-center justify-center px-2 text-center text-xs text-muted-foreground">
        {t('uploads.previewUnavailable')}
      </span>
    );
  }

  return (
    <img
      src={url}
      alt=""
      className="size-full object-cover"
      onError={() => {
        setFailed(true);
      }}
    />
  );
}

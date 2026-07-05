import { useEffect, useState } from 'react';

import { LoadingSpinner } from '@/components/ui/LoadingSpinner';
import { resolveMediaPreviewUrl } from '@/lib/api/uploads';
import { useI18n } from '@/lib/i18n/context';

export function ProductImagePreview({ fileId }: { fileId: string }) {
  const { t } = useI18n();
  const [url, setUrl] = useState('');
  const [loading, setLoading] = useState(true);
  const [failed, setFailed] = useState(false);

  useEffect(() => {
    let cancelled = false;
    let blobUrl: string | null = null;

    setLoading(true);
    setFailed(false);
    setUrl('');

    void resolveMediaPreviewUrl(fileId)
      .then((previewUrl) => {
        if (cancelled) {
          if (previewUrl.startsWith('blob:')) {
            URL.revokeObjectURL(previewUrl);
          }
          return;
        }
        if (previewUrl.startsWith('blob:')) {
          blobUrl = previewUrl;
        }
        setUrl(previewUrl);
      })
      .catch(() => {
        if (!cancelled) {
          setFailed(true);
        }
      })
      .finally(() => {
        if (!cancelled) {
          setLoading(false);
        }
      });

    return () => {
      cancelled = true;
      if (blobUrl) {
        URL.revokeObjectURL(blobUrl);
      }
    };
  }, [fileId]);

  if (loading) {
    return (
      <span className="absolute inset-0 flex items-center justify-center">
        <LoadingSpinner className="size-5" />
      </span>
    );
  }

  if (failed || !url) {
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
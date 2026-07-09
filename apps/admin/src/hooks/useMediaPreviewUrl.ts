import { useEffect, useState } from 'react';

import { resolveMediaPreviewUrl } from '@/lib/api/uploads';

/** Resolves a media file id to a URL safe for `<img src>` (blob or public route). */
export function useMediaPreviewUrl(fileId: string | undefined): string | null {
  const [previewUrl, setPreviewUrl] = useState<string | null>(null);

  useEffect(() => {
    if (!fileId) {
      setPreviewUrl(null);
      return;
    }

    let cancelled = false;
    let blobUrl: string | null = null;

    void resolveMediaPreviewUrl(fileId)
      .then((url) => {
        if (cancelled) {
          if (url.startsWith('blob:')) {
            URL.revokeObjectURL(url);
          }
          return;
        }
        if (url.startsWith('blob:')) {
          blobUrl = url;
        }
        setPreviewUrl(url);
      })
      .catch(() => {
        if (!cancelled) {
          setPreviewUrl(null);
        }
      });

    return () => {
      cancelled = true;
      if (blobUrl) {
        URL.revokeObjectURL(blobUrl);
      }
    };
  }, [fileId]);

  return previewUrl;
}

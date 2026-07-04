import { useEffect, useState } from 'react';

import { fetchMediaUrl } from '@/lib/api/uploads';

export function ProductImagePreview({ fileId }: { fileId: string }) {
  const [url, setUrl] = useState('');
  const [failed, setFailed] = useState(false);

  useEffect(() => {
    let cancelled = false;
    void fetchMediaUrl(fileId)
      .then((response) => {
        if (!cancelled) setUrl(response.url);
      })
      .catch(() => {
        if (!cancelled) setFailed(true);
      });
    return () => {
      cancelled = true;
    };
  }, [fileId]);

  if (failed || !url) {
    return <span className="text-xs text-muted-foreground">Preview unavailable</span>;
  }

  return <img src={url} alt="" className="size-full object-cover" />;
}

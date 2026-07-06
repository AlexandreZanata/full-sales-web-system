import { useEffect, useState } from 'react';

import { subscribeCatalogRevision } from '@/lib/catalog/useCatalogRealtime';

export function useCatalogRevision(): number {
  const [revision, setRevision] = useState(0);

  useEffect(() => {
    return subscribeCatalogRevision(() => {
      setRevision((value) => value + 1);
    });
  }, []);

  return revision;
}

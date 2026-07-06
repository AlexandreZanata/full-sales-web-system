import { QueryClient } from '@tanstack/react-query';
import { describe, expect, it } from 'vitest';

import { patchCategoryInListCaches } from '@/lib/catalog/patchCategoryListCache';
import type { CategorySummary } from '@/lib/api/types';

const snacks: CategorySummary = {
  id: 'cat-snacks',
  name: 'Snacks',
  slug: 'snacks',
  sortOrder: 1,
  active: true,
  imageFileId: 'file-old',
  thumbUrl: '/v1/public/media/file-old/content',
};

describe('patchCategoryInListCaches', () => {
  it('updates matching category rows in every categories list query', () => {
    const queryClient = new QueryClient();
    queryClient.setQueryData(['categories', ''], {
      data: [snacks],
      pagination: { has_more: false },
    });
    queryClient.setQueryData(['categories', 'picker'], {
      data: [snacks],
      pagination: { has_more: false },
    });

    patchCategoryInListCaches(queryClient, {
      ...snacks,
      imageFileId: 'file-new',
      thumbUrl: '/v1/public/media/file-new/content',
    });

    expect(queryClient.getQueryData(['categories', ''])).toEqual({
      data: [
        {
          ...snacks,
          imageFileId: 'file-new',
          thumbUrl: '/v1/public/media/file-new/content',
        },
      ],
      pagination: { has_more: false },
    });
    expect(queryClient.getQueryData(['categories', 'picker'])).toEqual({
      data: [
        {
          ...snacks,
          imageFileId: 'file-new',
          thumbUrl: '/v1/public/media/file-new/content',
        },
      ],
      pagination: { has_more: false },
    });
  });
});

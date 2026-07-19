import { useMutation, useQueryClient } from '@tanstack/react-query';

import { deactivateCategory, reorderCategories, updateCategory } from '@/lib/api/categories';
import type { MessageKey } from '@/lib/i18n/messages';
import { useToast } from '@/hooks/useToast';

type Translate = (key: MessageKey) => string;

export function useCategoryListMutations(t: Translate, onDeactivated: () => void) {
  const toast = useToast();
  const queryClient = useQueryClient();

  async function invalidate() {
    await queryClient.invalidateQueries({ queryKey: ['categories'] });
  }

  const reorderMutation = useMutation({
    mutationFn: reorderCategories,
    onSuccess: async () => {
      await invalidate();
      toast.success(t('categories.toast.reordered'));
    },
    onError: () => {
      toast.error(t('errors.actionFailed'));
    },
  });

  const deactivateMutation = useMutation({
    mutationFn: deactivateCategory,
    onSuccess: async () => {
      await invalidate();
      toast.success(t('categories.toast.deactivated'));
      onDeactivated();
    },
    onError: () => {
      toast.error(t('errors.actionFailed'));
    },
  });

  const reactivateMutation = useMutation({
    mutationFn: (id: string) => updateCategory(id, { active: true }),
    onSuccess: async () => {
      await invalidate();
      toast.success(t('categories.toast.reactivated'));
    },
    onError: () => {
      toast.error(t('errors.actionFailed'));
    },
  });

  return { reorderMutation, deactivateMutation, reactivateMutation, invalidate };
}

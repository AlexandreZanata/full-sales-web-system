import { useQueryClient } from '@tanstack/react-query';
import { useEffect, useState } from 'react';

import { FileUploadField } from '@/components/uploads/FileUploadField';
import { useToast } from '@/hooks/useToast';
import { uploadCategoryImage } from '@/lib/api/categories';
import type { CategoryDetail } from '@/lib/api/types';
import { patchCategoryInListCaches } from '@/lib/catalog/patchCategoryListCache';
import { onAdminCatalogChanged } from '@/lib/catalog/useCatalogRealtime';
import { useI18n } from '@/lib/i18n/context';

type CategoryImageSectionProps = {
  categoryId: string;
  imageFileId?: string;
  onImageUpdated: (category: CategoryDetail) => void;
};

export function CategoryImageSection({
  categoryId,
  imageFileId: initialImageFileId,
  onImageUpdated,
}: CategoryImageSectionProps) {
  const { t } = useI18n();
  const toast = useToast();
  const queryClient = useQueryClient();
  const [imageFileId, setImageFileId] = useState(initialImageFileId ?? '');
  const [saving, setSaving] = useState(false);

  useEffect(() => {
    setImageFileId(initialImageFileId ?? '');
  }, [initialImageFileId, categoryId]);

  async function handleImageChange(fileId: string) {
    setImageFileId(fileId);
    setSaving(true);
    try {
      const updated = await uploadCategoryImage(categoryId, fileId);
      patchCategoryInListCaches(queryClient, updated);
      onAdminCatalogChanged(queryClient);
      onImageUpdated(updated);
      toast.success(t('categories.toast.imageSaved'));
    } catch {
      toast.error(t('errors.actionFailed'));
      setImageFileId(initialImageFileId ?? '');
    } finally {
      setSaving(false);
    }
  }

  return (
    <div className="space-y-2">
      <FileUploadField
        label={t('categories.form.imageLabel')}
        fileId={imageFileId}
        onChange={(fileId) => void handleImageChange(fileId)}
        entityType="ProductCategory"
        entityId={categoryId}
        previewMode="public"
      />
      {saving ? (
        <p className="text-xs text-muted-foreground">{t('categories.form.imageSaving')}</p>
      ) : null}
    </div>
  );
}

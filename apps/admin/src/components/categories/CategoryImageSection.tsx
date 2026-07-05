import { useState } from 'react';

import { FileUploadField } from '@/components/uploads/FileUploadField';
import { useToast } from '@/hooks/useToast';
import { uploadCategoryImage } from '@/lib/api/categories';
import { useI18n } from '@/lib/i18n/context';

type CategoryImageSectionProps = {
  categoryId: string;
  imageFileId?: string;
  onImageUpdated: (fileId: string) => void;
};

export function CategoryImageSection({
  categoryId,
  imageFileId: initialImageFileId,
  onImageUpdated,
}: CategoryImageSectionProps) {
  const { t } = useI18n();
  const toast = useToast();
  const [imageFileId, setImageFileId] = useState(initialImageFileId ?? '');
  const [saving, setSaving] = useState(false);

  async function handleImageChange(fileId: string) {
    setImageFileId(fileId);
    setSaving(true);
    try {
      await uploadCategoryImage(categoryId, fileId);
      onImageUpdated(fileId);
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
      />
      {saving ? (
        <p className="text-xs text-muted-foreground">{t('categories.form.imageSaving')}</p>
      ) : null}
    </div>
  );
}

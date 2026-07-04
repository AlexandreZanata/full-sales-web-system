import { useState } from 'react';

import { ProductImagePreview } from '@/components/products/ProductImagePreview';
import { FileUploadField } from '@/components/uploads/FileUploadField';
import { Button } from '@/components/ui/Button';
import { Card } from '@/components/ui/Card';
import { useToast } from '@/hooks/useToast';
import { attachProductImage, deleteProductImage } from '@/lib/api/products';
import type { ProductImage } from '@/lib/api/types';
import { useI18n } from '@/lib/i18n/context';

type ProductImagesSectionProps = {
  productId: string;
};

export function ProductImagesSection({ productId }: ProductImagesSectionProps) {
  const { t } = useI18n();
  const toast = useToast();
  const [images, setImages] = useState<ProductImage[]>([]);
  const [isPrimaryUpload, setIsPrimaryUpload] = useState(false);
  const [attaching, setAttaching] = useState(false);
  const [uploadKey, setUploadKey] = useState(0);

  async function handleUploadComplete(fileId: string) {
    setAttaching(true);
    try {
      const image = await attachProductImage(productId, {
        fileId,
        isPrimary: isPrimaryUpload,
      });
      setImages((current) => {
        const withoutDupes = isPrimaryUpload
          ? current.map((item) => ({ ...item, isPrimary: false }))
          : current;
        return [...withoutDupes, image];
      });
      toast.success(t('products.toast.imageAttached'));
      setIsPrimaryUpload(false);
      setUploadKey((value) => value + 1);
    } catch {
      toast.error(t('errors.actionFailed'));
    } finally {
      setAttaching(false);
    }
  }

  async function handleDelete(image: ProductImage) {
    try {
      await deleteProductImage(productId, image.id);
      setImages((current) => current.filter((item) => item.id !== image.id));
      toast.success(t('products.toast.imageRemoved'));
    } catch {
      toast.error(t('errors.actionFailed'));
    }
  }

  async function handleSetPrimary(image: ProductImage) {
    try {
      await deleteProductImage(productId, image.id);
      const attached = await attachProductImage(productId, {
        fileId: image.fileId,
        isPrimary: true,
      });
      setImages((current) =>
        current
          .filter((item) => item.id !== image.id)
          .map((item) => ({ ...item, isPrimary: false }))
          .concat(attached),
      );
      toast.success(t('products.toast.primaryImageUpdated'));
    } catch {
      toast.error(t('errors.actionFailed'));
    }
  }

  return (
    <Card className="space-y-4">
      <p className="text-xs font-semibold uppercase tracking-[0.12em] text-muted-foreground">
        {t('products.images.title')}
      </p>
      <p className="text-sm text-muted-foreground">{t('products.images.uploadSessionHint')}</p>

      <label className="flex items-center gap-2 text-sm text-foreground">
        <input
          type="checkbox"
          checked={isPrimaryUpload}
          onChange={(event) => {
            setIsPrimaryUpload(event.target.checked);
          }}
        />
        {t('products.images.setPrimaryUpload')}
      </label>

      <FileUploadField
        key={uploadKey}
        label={t('products.images.label')}
        fileId=""
        onChange={(fileId) => void handleUploadComplete(fileId)}
        entityType="Product"
        entityId={productId}
      />
      {attaching ? (
        <p className="text-xs text-muted-foreground">{t('products.images.attaching')}</p>
      ) : null}

      {images.length > 0 ? (
        <ul className="grid gap-3 sm:grid-cols-2">
          {images.map((image) => (
            <li
              key={image.id}
              className="flex flex-col gap-3 rounded-lg border border-hairline p-3"
            >
              <div className="flex size-24 items-center justify-center overflow-hidden rounded-md border border-hairline bg-surface">
                <ProductImagePreview fileId={image.fileId} />
              </div>
              <p className="text-xs text-muted-foreground">
                {image.isPrimary ? t('products.images.primary') : t('products.images.secondary')}
              </p>
              <div className="flex flex-wrap gap-2">
                {!image.isPrimary ? (
                  <Button
                    type="button"
                    variant="secondary"
                    onClick={() => void handleSetPrimary(image)}
                  >
                    {t('products.images.setPrimary')}
                  </Button>
                ) : null}
                <Button type="button" variant="danger" onClick={() => void handleDelete(image)}>
                  {t('products.images.remove')}
                </Button>
              </div>
            </li>
          ))}
        </ul>
      ) : null}
    </Card>
  );
}

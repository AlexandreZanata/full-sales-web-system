import { useQuery, useQueryClient } from '@tanstack/react-query';
import { useState } from 'react';

import { ProductImagePreview } from '@/components/products/ProductImagePreview';
import { FileUploadField } from '@/components/uploads/FileUploadField';
import { Button } from '@/components/ui/Button';
import { Card } from '@/components/ui/Card';
import { LoadingSpinner } from '@/components/ui/LoadingSpinner';
import { useToast } from '@/hooks/useToast';
import { attachProductImage, deleteProductImage, fetchProductImages } from '@/lib/api/products';
import type { ProductImage } from '@/lib/api/types';
import { useI18n } from '@/lib/i18n/context';

type ProductImagesSectionProps = {
  productId: string;
};

function productImagesQueryKey(productId: string) {
  return ['products', productId, 'images'] as const;
}

export function ProductImagesSection({ productId }: ProductImagesSectionProps) {
  const { t } = useI18n();
  const toast = useToast();
  const queryClient = useQueryClient();
  const [isPrimaryUpload, setIsPrimaryUpload] = useState(false);
  const [attaching, setAttaching] = useState(false);
  const [uploadKey, setUploadKey] = useState(0);

  const imagesQuery = useQuery({
    queryKey: productImagesQueryKey(productId),
    queryFn: () => fetchProductImages(productId),
  });

  const images = imagesQuery.data?.items ?? [];

  async function invalidateImages() {
    await queryClient.invalidateQueries({ queryKey: productImagesQueryKey(productId) });
  }

  async function handleUploadComplete(fileId: string) {
    setAttaching(true);
    try {
      await attachProductImage(productId, {
        fileId,
        isPrimary: isPrimaryUpload,
      });
      await invalidateImages();
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
      await invalidateImages();
      toast.success(t('products.toast.imageRemoved'));
    } catch {
      toast.error(t('errors.actionFailed'));
    }
  }

  async function handleSetPrimary(image: ProductImage) {
    try {
      await deleteProductImage(productId, image.id);
      await attachProductImage(productId, {
        fileId: image.fileId,
        isPrimary: true,
      });
      await invalidateImages();
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
      <p className="text-sm text-muted-foreground">{t('products.images.uploadHint')}</p>

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

      {imagesQuery.isLoading ? (
        <div className="flex justify-center py-8">
          <LoadingSpinner />
        </div>
      ) : imagesQuery.isError ? (
        <p className="text-sm text-destructive">{t('products.images.loadError')}</p>
      ) : images.length === 0 ? (
        <p className="text-sm text-muted-foreground">{t('products.images.empty')}</p>
      ) : (
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
      )}
    </Card>
  );
}

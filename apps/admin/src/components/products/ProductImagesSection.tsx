import { useQuery, useQueryClient } from '@tanstack/react-query';
import { useState } from 'react';

import { ProductImageSlot } from '@/components/products/ProductImageSlot';
import { imagesBySlot, slotCount } from '@/components/products/productImageSlots';
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

  const imagesQuery = useQuery({
    queryKey: productImagesQueryKey(productId),
    queryFn: () => fetchProductImages(productId),
  });

  const images = imagesQuery.data?.data ?? [];
  const slots = slotCount(images);
  const slotImages = imagesBySlot(images, slots);

  async function invalidateImages() {
    await queryClient.invalidateQueries({ queryKey: productImagesQueryKey(productId) });
  }

  async function handleSlotUpload(fileId: string, slotIndex: number, existingImage?: ProductImage) {
    try {
      if (existingImage) {
        const wasPrimary = existingImage.isPrimary;
        await deleteProductImage(productId, existingImage.id);
        await attachProductImage(productId, {
          fileId,
          isPrimary: wasPrimary,
          sortOrder: slotIndex,
        });
      } else {
        const isFirstImage = images.length === 0;
        await attachProductImage(productId, {
          fileId,
          isPrimary: isPrimaryUpload || isFirstImage,
          sortOrder: slotIndex,
        });
      }

      await invalidateImages();
      toast.success(t('products.toast.imageAttached'));
      if (!existingImage) {
        setIsPrimaryUpload(false);
      }
    } catch {
      toast.error(t('errors.actionFailed'));
      throw new Error('attach failed');
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
        sortOrder: image.sortOrder,
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

      {imagesQuery.isLoading ? (
        <div className="flex justify-center py-8">
          <LoadingSpinner />
        </div>
      ) : imagesQuery.isError ? (
        <p className="text-sm text-destructive">{t('products.images.loadError')}</p>
      ) : (
        <ul className="grid gap-3 sm:grid-cols-2 lg:grid-cols-3">
          {slotImages.map((image, slotIndex) => (
            <ProductImageSlot
              key={`slot-${String(slotIndex)}`}
              productId={productId}
              image={image}
              onUploadComplete={(fileId) => handleSlotUpload(fileId, slotIndex, image)}
              onDelete={image ? () => handleDelete(image) : undefined}
              onSetPrimary={image ? () => handleSetPrimary(image) : undefined}
            />
          ))}
        </ul>
      )}
    </Card>
  );
}

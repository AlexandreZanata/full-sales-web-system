import { useState } from 'react';

import { ProductImagePreview } from '@/components/products/ProductImagePreview';
import { FileUploadField } from '@/components/uploads/FileUploadField';
import { Button } from '@/components/ui/Button';
import { Card } from '@/components/ui/Card';
import { useToast } from '@/hooks/useToast';
import { ApiError } from '@/lib/api/client';
import { attachProductImage, deleteProductImage } from '@/lib/api/products';
import type { ProductImage } from '@/lib/api/types';
import { formatApiErrorMessage } from '@/lib/utils';

type ProductImagesSectionProps = {
  productId: string;
};

export function ProductImagesSection({ productId }: ProductImagesSectionProps) {
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
      toast.success('Image attached');
      setIsPrimaryUpload(false);
      setUploadKey((value) => value + 1);
    } catch (error) {
      const message =
        error instanceof ApiError
          ? formatApiErrorMessage(error.message, error.code)
          : 'Unable to attach image';
      toast.error(message);
    } finally {
      setAttaching(false);
    }
  }

  async function handleDelete(image: ProductImage) {
    try {
      await deleteProductImage(productId, image.id);
      setImages((current) => current.filter((item) => item.id !== image.id));
      toast.success('Image removed');
    } catch (error) {
      const message =
        error instanceof ApiError
          ? formatApiErrorMessage(error.message, error.code)
          : 'Unable to remove image';
      toast.error(message);
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
      toast.success('Primary image updated');
    } catch (error) {
      const message =
        error instanceof ApiError
          ? formatApiErrorMessage(error.message, error.code)
          : 'Unable to set primary image';
      toast.error(message);
    }
  }

  return (
    <Card className="space-y-4">
      <p className="text-xs font-semibold uppercase tracking-[0.12em] text-muted-foreground">
        Product images
      </p>
      <p className="text-sm text-muted-foreground">
        Upload images for this product. Images added in this session appear below.
      </p>

      <label className="flex items-center gap-2 text-sm text-foreground">
        <input
          type="checkbox"
          checked={isPrimaryUpload}
          onChange={(event) => {
            setIsPrimaryUpload(event.target.checked);
          }}
        />
        Set uploaded image as primary
      </label>

      <FileUploadField
        key={uploadKey}
        label="Product image"
        fileId=""
        onChange={(fileId) => void handleUploadComplete(fileId)}
        entityType="Product"
        entityId={productId}
      />
      {attaching ? <p className="text-xs text-muted-foreground">Attaching image…</p> : null}

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
                {image.isPrimary ? 'Primary' : 'Secondary'}
              </p>
              <div className="flex flex-wrap gap-2">
                {!image.isPrimary ? (
                  <Button
                    type="button"
                    variant="secondary"
                    onClick={() => void handleSetPrimary(image)}
                  >
                    Set primary
                  </Button>
                ) : null}
                <Button type="button" variant="danger" onClick={() => void handleDelete(image)}>
                  Remove
                </Button>
              </div>
            </li>
          ))}
        </ul>
      ) : null}
    </Card>
  );
}

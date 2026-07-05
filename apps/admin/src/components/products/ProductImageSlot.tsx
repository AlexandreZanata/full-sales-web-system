import { useRef, useState } from 'react';

import { ProductImagePreview } from '@/components/products/ProductImagePreview';
import { Button } from '@/components/ui/Button';
import { useToast } from '@/hooks/useToast';
import { ApiError } from '@/lib/api/client';
import { uploadMediaFile } from '@/lib/api/uploads';
import type { ProductImage } from '@/lib/api/types';
import { useI18n } from '@/lib/i18n/context';
import { IMAGE_UPLOAD_ACCEPT } from '@/lib/uploadAccept';
import { formatApiErrorMessage } from '@/lib/utils';

type ProductImageSlotProps = {
  productId: string;
  image?: ProductImage;
  onUploadComplete: (fileId: string) => Promise<void>;
  onDelete?: () => Promise<void>;
  onSetPrimary?: () => Promise<void>;
};

export function ProductImageSlot({
  productId,
  image,
  onUploadComplete,
  onDelete,
  onSetPrimary,
}: ProductImageSlotProps) {
  const { t } = useI18n();
  const toast = useToast();
  const inputRef = useRef<HTMLInputElement>(null);
  const [isUploading, setIsUploading] = useState(false);
  const [localPreview, setLocalPreview] = useState<string | null>(null);
  const [uploadError, setUploadError] = useState<string | null>(null);

  async function handleFileChange(event: React.ChangeEvent<HTMLInputElement>) {
    const file = event.target.files?.[0];
    if (!file) return;

    const nextLocalPreview = URL.createObjectURL(file);
    setLocalPreview((current) => {
      if (current) {
        URL.revokeObjectURL(current);
      }
      return nextLocalPreview;
    });
    setUploadError(null);
    setIsUploading(true);

    try {
      const response = await uploadMediaFile(file, 'Product', productId);
      await onUploadComplete(response.id);
      setLocalPreview((current) => {
        if (current) {
          URL.revokeObjectURL(current);
        }
        return null;
      });
    } catch (caught) {
      setLocalPreview((current) => {
        if (current) {
          URL.revokeObjectURL(current);
        }
        return null;
      });
      const message =
        caught instanceof ApiError
          ? formatApiErrorMessage(caught.message, caught.code)
          : t('errors.uploadFailed');
      setUploadError(message);
      toast.error(message);
    } finally {
      setIsUploading(false);
      if (inputRef.current) {
        inputRef.current.value = '';
      }
    }
  }

  const previewSrc = isUploading && localPreview ? localPreview : null;

  return (
    <li className="flex flex-col gap-3 rounded-lg border border-hairline p-3">
      <button
        type="button"
        className="relative aspect-square w-full overflow-hidden rounded-md border border-hairline bg-surface transition hover:border-foreground/30 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
        disabled={isUploading}
        onClick={() => inputRef.current?.click()}
        aria-label={image ? t('products.images.replaceImage') : t('products.images.clickToUpload')}
      >
        {previewSrc ? (
          <img src={previewSrc} alt="" className="size-full object-cover" />
        ) : image ? (
          <ProductImagePreview fileId={image.fileId} />
        ) : (
          <span className="absolute inset-0 flex items-center justify-center px-3 text-center text-xs text-muted-foreground">
            {isUploading ? t('uploads.uploading') : t('products.images.clickToUpload')}
          </span>
        )}
      </button>

      <input
        ref={inputRef}
        type="file"
        accept={IMAGE_UPLOAD_ACCEPT}
        className="hidden"
        onChange={(event) => void handleFileChange(event)}
      />

      <p className="text-xs text-muted-foreground">
        {image
          ? image.isPrimary
            ? t('products.images.primary')
            : t('products.images.secondary')
          : t('products.images.emptySlot')}
      </p>

      {uploadError ? <p className="text-xs text-destructive">{uploadError}</p> : null}

      <div className="flex flex-wrap gap-2">
        <Button
          type="button"
          variant="secondary"
          disabled={isUploading}
          onClick={() => inputRef.current?.click()}
        >
          {isUploading ? t('uploads.uploading') : t('uploads.uploadFile')}
        </Button>
        {image && !image.isPrimary && onSetPrimary ? (
          <Button
            type="button"
            variant="secondary"
            disabled={isUploading}
            onClick={() => void onSetPrimary()}
          >
            {t('products.images.setPrimary')}
          </Button>
        ) : null}
        {image && onDelete ? (
          <Button
            type="button"
            variant="danger"
            disabled={isUploading}
            onClick={() => void onDelete()}
          >
            {t('products.images.remove')}
          </Button>
        ) : null}
      </div>
    </li>
  );
}

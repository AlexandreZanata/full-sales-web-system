import { useEffect, useRef, useState } from 'react';

import { Button } from '@/components/ui/Button';
import { useToast } from '@/hooks/useToast';
import { ApiError } from '@/lib/api/client';
import { resolveMediaPreviewUrl, uploadMediaFile, type MediaEntityType } from '@/lib/api/uploads';
import { useI18n } from '@/lib/i18n/context';
import { translateFormError } from '@/lib/i18n/labels';
import { IMAGE_UPLOAD_ACCEPT, IMAGE_UPLOAD_HINT } from '@/lib/uploadAccept';
import { formatApiErrorMessage } from '@/lib/utils';

type FileUploadFieldProps = {
  label: string;
  fileId: string;
  onChange: (fileId: string) => void;
  entityType: MediaEntityType;
  entityId: string;
  error?: string;
};

export function FileUploadField({
  label,
  fileId,
  onChange,
  entityType,
  entityId,
  error,
}: FileUploadFieldProps) {
  const { t } = useI18n();
  const inputRef = useRef<HTMLInputElement>(null);
  const toast = useToast();
  const [isUploading, setIsUploading] = useState(false);
  const [uploadError, setUploadError] = useState<string | null>(null);
  const [localPreview, setLocalPreview] = useState<string | null>(null);
  const [remotePreviewUrl, setRemotePreviewUrl] = useState('');
  const [remotePreviewFailed, setRemotePreviewFailed] = useState(false);

  useEffect(() => {
    if (!fileId) {
      setRemotePreviewUrl('');
      return;
    }

    let cancelled = false;
    let blobUrl: string | null = null;

    void resolveMediaPreviewUrl(fileId)
      .then((previewUrl) => {
        if (cancelled) {
          if (previewUrl.startsWith('blob:')) {
            URL.revokeObjectURL(previewUrl);
          }
          return;
        }
        if (previewUrl.startsWith('blob:')) {
          blobUrl = previewUrl;
        }
        setRemotePreviewUrl(previewUrl);
        setRemotePreviewFailed(false);
      })
      .catch(() => {
        if (!cancelled) {
          setRemotePreviewUrl('');
          setRemotePreviewFailed(true);
        }
      });

    return () => {
      cancelled = true;
      if (blobUrl) {
        URL.revokeObjectURL(blobUrl);
      }
    };
  }, [fileId]);

  useEffect(() => {
    return () => {
      if (localPreview) {
        URL.revokeObjectURL(localPreview);
      }
    };
  }, [localPreview]);

  async function handleFileChange(event: React.ChangeEvent<HTMLInputElement>) {
    const file = event.target.files?.[0];
    if (!file) return;

    const nextLocalPreview = URL.createObjectURL(file);
    setLocalPreview((current) => {
      if (current) URL.revokeObjectURL(current);
      return nextLocalPreview;
    });
    setUploadError(null);
    setRemotePreviewFailed(false);
    setIsUploading(true);

    try {
      const response = await uploadMediaFile(file, entityType, entityId);
      onChange(response.id);
      toast.success(t('uploads.fileUploaded'));
    } catch (caught) {
      const message =
        caught instanceof ApiError
          ? formatApiErrorMessage(caught.message, caught.code)
          : t('errors.uploadFailed');
      setUploadError(message);
      toast.error(message);
    } finally {
      setIsUploading(false);
      if (inputRef.current) inputRef.current.value = '';
    }
  }

  const previewSrc = localPreview ?? (remotePreviewFailed ? '' : remotePreviewUrl);
  const showImagePreview = previewSrc.length > 0;

  return (
    <div className="space-y-3">
      <div className="flex flex-col gap-4 sm:flex-row sm:items-start">
        <div
          className="flex size-32 shrink-0 items-center justify-center overflow-hidden rounded-lg border border-hairline bg-surface"
          data-testid="upload-preview"
        >
          {showImagePreview ? (
            <img
              src={previewSrc}
              alt=""
              className="size-full object-cover"
              onError={() => {
                if (localPreview) return;
                setRemotePreviewFailed(true);
              }}
            />
          ) : (
            <span className="px-3 text-center text-xs text-muted-foreground">
              {isUploading ? t('uploads.uploading') : t('uploads.noImage')}
            </span>
          )}
        </div>

        <div className="min-w-0 flex-1 space-y-3">
          <p className="text-xs font-semibold uppercase tracking-[0.12em] text-muted-foreground">
            {label}
          </p>
          <p className="text-sm text-muted-foreground">
            {fileId ? t('uploads.fileId').replace('{id}', fileId) : t('uploads.noFile')}
          </p>
          <div className="flex flex-wrap items-center gap-2">
            <input
              ref={inputRef}
              type="file"
              accept={IMAGE_UPLOAD_ACCEPT}
              className="hidden"
              onChange={(event) => void handleFileChange(event)}
            />
            <Button
              type="button"
              variant="secondary"
              disabled={isUploading || !entityId}
              onClick={() => inputRef.current?.click()}
            >
              {isUploading ? t('uploads.uploading') : t('uploads.uploadFile')}
            </Button>
            <p className="text-xs text-muted-foreground">{IMAGE_UPLOAD_HINT}</p>
            {uploadError ? <p className="text-xs text-destructive">{uploadError}</p> : null}
            {error ? (
              <p className="text-xs text-destructive">{translateFormError(t, error)}</p>
            ) : null}
            {remotePreviewFailed && fileId ? (
              <p className="text-xs text-destructive">{t('uploads.previewUnavailable')}</p>
            ) : null}
          </div>
        </div>
      </div>
    </div>
  );
}

import { apiRequest, parseApiErrorBody, ApiError } from '@/lib/api/client';

export type MediaEntityType =
  | 'Product'
  | 'ProductCategory'
  | 'User'
  | 'Commerce'
  | 'Delivery'
  | 'Tenant'
  | 'PortalBanner';

export type MediaUploadResponse = {
  id: string;
  entityType: MediaEntityType;
  entityId: string;
  mimeType: string;
  sizeBytes: number;
  sha256: string;
};

export type MediaUrlResponse = {
  url: string;
  expiresAt: string;
};

export async function uploadMediaFile(
  file: File,
  entityType: MediaEntityType,
  entityId: string,
): Promise<MediaUploadResponse> {
  const formData = new FormData();
  formData.append('file', file);
  formData.append('entityType', entityType);
  formData.append('entityId', entityId);

  const response = await apiRequest('/media/upload', {
    method: 'POST',
    body: formData,
  });

  if (!response.ok) {
    const body = await parseApiErrorBody(response);
    throw new ApiError(response.status, body);
  }

  return (await response.json()) as MediaUploadResponse;
}

export async function fetchMediaUrl(fileId: string): Promise<MediaUrlResponse> {
  const response = await apiRequest(`/media/${fileId}/url`);

  if (!response.ok) {
    const body = await parseApiErrorBody(response);
    throw new ApiError(response.status, body);
  }

  return (await response.json()) as MediaUrlResponse;
}

function isMemoryPresignedUrl(url: string): boolean {
  return url.startsWith('memory://');
}

export function resolvePublicProductMediaUrl(fileId: string): string {
  return `/v1/public/media/${fileId}/content`;
}

export function resolveMediaContentUrl(fileId: string, presignedUrl: string): string {
  if (!isMemoryPresignedUrl(presignedUrl)) {
    return presignedUrl;
  }
  return `/v1/media/${fileId}/content`;
}

function isAuthenticatedMediaPath(url: string): boolean {
  return url.startsWith('/v1/media/');
}

async function fetchAuthenticatedMediaBlobUrl(fileId: string): Promise<string> {
  const response = await apiRequest(`/media/${fileId}/content`);
  if (!response.ok) {
    const body = await parseApiErrorBody(response);
    throw new ApiError(response.status, body);
  }
  return URL.createObjectURL(await response.blob());
}

/** Product catalog images — safe for `<img src>` (public route, no auth header). */
export function resolveProductImagePreviewUrl(fileId: string): string {
  return resolvePublicProductMediaUrl(fileId);
}

/** Category catalog images — same public route as product gallery (Phase 43). */
export function resolveCatalogImagePreviewUrl(fileId: string): string {
  return resolvePublicProductMediaUrl(fileId);
}

/** Prefer API thumbUrl; fall back to public content route from file id. */
export function resolveCategoryThumbUrl(
  imageFileId?: string,
  thumbUrl?: string,
): string | undefined {
  if (thumbUrl) {
    return thumbUrl;
  }
  if (imageFileId) {
    return resolveCatalogImagePreviewUrl(imageFileId);
  }
  return undefined;
}

/** Bust browser cache when catalog media changes but the path stays the same. */
export function withCatalogImageCacheBust(url: string, cacheKey: string, revision = 0): string {
  const token = `${cacheKey}:${String(revision)}`;
  const separator = url.includes('?') ? '&' : '?';
  return `${url}${separator}v=${encodeURIComponent(token)}`;
}

export async function resolveMediaPreviewUrl(fileId: string): Promise<string> {
  const { url } = await fetchMediaUrl(fileId);
  const contentUrl = resolveMediaContentUrl(fileId, url);

  if (isAuthenticatedMediaPath(contentUrl)) {
    return fetchAuthenticatedMediaBlobUrl(fileId);
  }

  return contentUrl;
}

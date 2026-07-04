import { apiRequest, parseApiErrorBody, ApiError } from '@/lib/api/client';

export type MediaEntityType = 'Product' | 'User' | 'Commerce' | 'Delivery' | 'Tenant';

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

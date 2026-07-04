import { apiRequest, parseApiErrorBody, ApiError } from '@/lib/api/client';

export type MediaUploadResponse = {
  id: string;
  entityType: string;
  entityId: string;
  mimeType: string;
  sizeBytes: number;
  sha256: string;
};

export async function uploadDeliveryProof(
  file: File,
  deliveryId: string,
): Promise<MediaUploadResponse> {
  const formData = new FormData();
  formData.append('file', file);
  formData.append('entityType', 'Delivery');
  formData.append('entityId', deliveryId);

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

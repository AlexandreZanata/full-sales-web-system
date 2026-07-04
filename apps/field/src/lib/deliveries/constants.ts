export type DeliveryStatus = 'Waiting' | 'InTransit' | 'Delivered' | 'Failed';

export const DELIVERY_STATUSES: DeliveryStatus[] = ['Waiting', 'InTransit', 'Delivered', 'Failed'];

export type DeliveryStatusFilter = DeliveryStatus | '';

export function deliveryActionErrorMessage(code: string): string {
  if (code === 'PROOF_REQUIRED') {
    return 'Foto de comprovante obrigatória para confirmar a entrega.';
  }
  if (code === 'INVALID_DELIVERY_TRANSITION') {
    return 'Esta entrega não pode avançar para o próximo status.';
  }
  return 'Não foi possível concluir a operação.';
}

import { useState } from 'react';

import { ApiError } from '@/lib/api/client';
import { cancelSale, confirmSale } from '@/lib/api/sales';
import type { SaleDetail } from '@/lib/api/types';
import { saleActionErrorKey } from '@/lib/i18n/labels';
import type { MessageKey } from '@/lib/i18n/messages';
import { findInsufficientSaleProductId } from '@/lib/sales/findInsufficientSaleProductId';

type Translate = (key: MessageKey) => string;

type SaleDetailActionsParams = {
  saleId: string;
  t: Translate;
  invalidateSale: () => Promise<void>;
  onSuccess: (message: string) => void;
  onError: (message: string) => void;
};

export function useSaleDetailActions({
  saleId,
  t,
  invalidateSale,
  onSuccess,
  onError,
}: SaleDetailActionsParams) {
  const [actionLoading, setActionLoading] = useState(false);
  const [stockShortProductId, setStockShortProductId] = useState<string | null>(null);

  async function handleConfirm(detail: SaleDetail) {
    setActionLoading(true);
    setStockShortProductId(null);
    try {
      await confirmSale(saleId);
      await invalidateSale();
      onSuccess(t('sales.toast.confirmed'));
    } catch (error) {
      if (error instanceof ApiError && error.code === 'INSUFFICIENT_STOCK') {
        const productId = await findInsufficientSaleProductId(detail.items);
        setStockShortProductId(productId ?? null);
        return;
      }
      onError(
        error instanceof ApiError ? t(saleActionErrorKey(error.code)) : t('errors.actionFailed'),
      );
    } finally {
      setActionLoading(false);
    }
  }

  async function handleCancel(onDone: () => void) {
    setActionLoading(true);
    try {
      await cancelSale(saleId);
      await invalidateSale();
      onSuccess(t('sales.toast.cancelled'));
      onDone();
    } catch (error) {
      onError(
        error instanceof ApiError ? t(saleActionErrorKey(error.code)) : t('errors.actionFailed'),
      );
    } finally {
      setActionLoading(false);
    }
  }

  return { actionLoading, stockShortProductId, handleConfirm, handleCancel };
}

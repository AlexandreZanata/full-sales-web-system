import type { DeliveryStatus, OrderStatus, SaleStatus } from '@/lib/admin-tokens';
import type { ReportType } from '@/lib/api/types';
import type { MessageKey } from '@/lib/i18n/messages';
import type { ActiveFilter, AddressTypeOption } from '@/lib/commerces/constants';
import type { DeliveryStatusFilter } from '@/lib/deliveries/constants';
import type { OrderStatusFilter } from '@/lib/orders/constants';
import type { SaleStatusFilter } from '@/lib/sales/constants';
import type { UserRoleOption } from '@/lib/users/constants';
import type { PaymentMethodValue } from '@/lib/sales/constants';

type Translate = (key: MessageKey) => string;

export function translateOrderStatus(t: Translate, status: OrderStatus): string {
  return t(`status.order.${status}`);
}

export function translateSaleStatus(t: Translate, status: SaleStatus): string {
  return t(`status.sale.${status}`);
}

export function translateDeliveryStatus(t: Translate, status: DeliveryStatus): string {
  return t(`status.delivery.${status}`);
}

export function translateReportType(t: Translate, reportType: ReportType): string {
  return t(`status.report.${reportType}`);
}

export function translateRole(t: Translate, role: UserRoleOption): string {
  return t(`role.${role}`);
}

export function translateAddressType(t: Translate, type: AddressTypeOption): string {
  return t(`addressType.${type}`);
}

export function translatePaymentMethod(t: Translate, value: string): string {
  const normalized = value.trim().toLowerCase();
  if (normalized === 'notdeclared') {
    return t('payment.notDeclared');
  }
  if (normalized === 'cash') return t('payment.cash');
  if (normalized === 'pix') return t('payment.pix');
  if (normalized === 'credit') return t('payment.credit');
  if (normalized === 'debit') return t('payment.debit');
  return value;
}

export function translateDeclaredPayment(t: Translate, method: string, received: boolean): string {
  const normalized = method.trim().toLowerCase();
  if (!normalized || normalized === 'notdeclared') {
    return t('payment.notDeclared');
  }
  const label = translatePaymentMethod(t, method);
  return received
    ? t('payment.declaredReceived').replace('{method}', label)
    : t('payment.declaredPending').replace('{method}', label);
}

export function orderStatusFilterLabel(t: Translate, value: OrderStatusFilter): string {
  if (value === '') return t('common.filter.allStatuses');
  return translateOrderStatus(t, value);
}

export function saleStatusFilterLabel(t: Translate, value: SaleStatusFilter): string {
  if (value === '') return t('common.filter.allStatuses');
  return translateSaleStatus(t, value);
}

export function deliveryStatusFilterLabel(t: Translate, value: DeliveryStatusFilter): string {
  if (value === '') return t('common.filter.allStatuses');
  return translateDeliveryStatus(t, value);
}

export function activeFilterLabel(t: Translate, value: ActiveFilter): string {
  if (value === '') return t('common.filter.allStatuses');
  if (value === 'true') return t('common.filter.activeOnly');
  return t('common.filter.inactiveOnly');
}

export function paymentMethodOptionLabel(t: Translate, value: PaymentMethodValue): string {
  return translatePaymentMethod(t, value);
}

export function formatPaginationSummary(
  t: Translate,
  state: { page: number; totalPages: number; total: number },
): string {
  return t('common.pagination.summary')
    .replace('{page}', String(state.page))
    .replace('{totalPages}', String(state.totalPages))
    .replace('{total}', String(state.total));
}

export function translateFormError(t: Translate, error: string | undefined): string | undefined {
  if (!error) return undefined;
  if (error.includes('.')) {
    return t(error as MessageKey);
  }
  return error;
}

export function orderActionErrorKey(code: string): MessageKey {
  switch (code) {
    case 'INSUFFICIENT_STOCK':
      return 'errors.orders.insufficientStock';
    case 'INVALID_ORDER_TRANSITION':
      return 'errors.orders.invalidTransition';
    case 'REJECTION_REASON_REQUIRED':
      return 'errors.orders.rejectionReasonRequired';
    case 'DELIVERY_EXISTS':
      return 'errors.orders.deliveryExists';
    default:
      return 'errors.actionFailed';
  }
}

export function saleActionErrorKey(code: string): MessageKey {
  switch (code) {
    case 'INSUFFICIENT_STOCK':
      return 'errors.sales.insufficientStock';
    case 'INVALID_TRANSITION':
      return 'errors.sales.invalidTransition';
    case 'INACTIVE_COMMERCE':
      return 'errors.sales.inactiveCommerce';
    case 'INACTIVE_PRODUCT':
      return 'errors.sales.inactiveProduct';
    case 'PRODUCT_NOT_FOUND':
      return 'errors.sales.productNotFound';
    case 'COMMERCE_NOT_FOUND':
      return 'errors.sales.commerceNotFound';
    default:
      return 'errors.actionFailed';
  }
}

export function reportActionErrorKey(code: string): MessageKey {
  switch (code) {
    case 'SIGNING_KEY_UNAVAILABLE':
      return 'errors.reports.signingKeyUnavailable';
    case 'VALIDATION_ERROR':
      return 'errors.reports.validationError';
    default:
      return 'errors.reports.generateFailed';
  }
}

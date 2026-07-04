export type PaginationMeta = {
  page: number;
  pageSize: number;
  total: number;
};

export type PaginatedResponse<T> = PaginationMeta & {
  items: T[];
};

export type OrderSummary = {
  id: string;
  status: string;
  commerceId: string;
  totalAmount: number;
  totalCurrency: string;
  createdAt: string;
};

export type SaleSummary = {
  id: string;
  commerceId: string;
  driverId: string;
  status: string;
  paymentMethod: string;
  totalAmount: number;
  totalCurrency: string;
  createdAt: string;
};

export type DeliverySummary = {
  id: string;
  orderId: string;
  driverId: string;
  status: string;
  saleId?: string;
};

export type TokenResponse = {
  accessToken: string;
  refreshToken: string;
  expiresIn: number;
};

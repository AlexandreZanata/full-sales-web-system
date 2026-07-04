export type PaginationMeta = {
  page: number;
  pageSize: number;
  total: number;
};

export type PaginatedResponse<T> = PaginationMeta & {
  items: T[];
};

export type CommerceSummary = {
  id: string;
  tradeName?: string;
  legalName: string;
  active: boolean;
};

export type ProductSummary = {
  id: string;
  name: string;
  sku: string;
  priceAmount: number;
  priceCurrency: string;
  active: boolean;
};

export type StockBalance = {
  productId: string;
  available: number;
  asOf: string;
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

export type SaleItem = {
  productId: string;
  quantity: number;
  unitPriceAmount: number;
  unitPriceCurrency: string;
  lineTotalAmount: number;
};

export type SaleDetail = {
  id: string;
  commerceId: string;
  driverId: string;
  status: string;
  paymentMethod: string;
  totalAmount: number;
  totalCurrency: string;
  items: SaleItem[];
};

export type CreateSaleRequest = {
  commerceId: string;
  items: Array<{ productId: string; quantity: number }>;
  paymentMethod: string;
};

export type TokenResponse = {
  accessToken: string;
  refreshToken: string;
  expiresIn: number;
};

export type SaleStatus = 'Pending' | 'Confirmed' | 'Cancelled';

export type PaymentMethod = 'cash' | 'pix' | 'credit' | 'debit';

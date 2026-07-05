export type PaginationMeta = {
  page: number;
  pageSize: number;
  total: number;
};

export type PaginatedResponse<T> = PaginationMeta & {
  items: T[];
};

export type PortalProduct = {
  id: string;
  name: string;
  sku: string;
  priceAmount: number;
  priceCurrency: string;
  categoryId?: string;
  categoryName?: string;
  categorySlug?: string;
  primaryImageUrl?: string;
};

export type PortalCategory = {
  id: string;
  name: string;
  slug: string;
  description?: string;
  sortOrder: number;
  active: boolean;
  imageFileId?: string;
  thumbUrl?: string;
  productCount?: number;
};

export type PortalCategoryWithProducts = PortalCategory & {
  products: PortalProduct[];
  page: number;
  pageSize: number;
  total: number;
};

export type PortalOrderItem = {
  id: string;
  productId: string;
  quantity: number;
  unitPriceAmount: number;
  unitPriceCurrency: string;
  lineTotalAmount: number;
};

export type PortalOrderSummary = {
  id: string;
  status: string;
  totalAmount: number;
  totalCurrency: string;
  createdAt: string;
};

export type PortalOrderDetail = {
  id: string;
  status: string;
  deliveryAddressId: string;
  notes?: string;
  totalAmount: number;
  totalCurrency: string;
  rejectionReason?: string;
  items: PortalOrderItem[];
};

export type CreatePortalOrderRequest = {
  deliveryAddressId: string;
  notes?: string;
  items: Array<{ productId: string; quantity: number }>;
};

export type TokenResponse = {
  accessToken: string;
  refreshToken: string;
  expiresIn: number;
};

export type OrderStatus =
  | 'Draft'
  | 'PendingApproval'
  | 'Approved'
  | 'Rejected'
  | 'Picking'
  | 'InTransit'
  | 'Delivered'
  | 'PartiallyDelivered'
  | 'Cancelled';

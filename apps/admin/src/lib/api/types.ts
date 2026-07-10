export type PaginationMeta = {
  page: number;
  pageSize: number;
  total: number;
};

export type PaginatedResponse<T> = PaginationMeta & {
  items: T[];
};

export type { CursorListResponse, CursorPaginationMeta } from '@/lib/cursorPagination';

export type OrderSummary = {
  id: string;
  status: string;
  commerceId: string;
  totalAmount: number;
  totalCurrency: string;
  createdAt: string;
};

export type OrderItem = {
  id: string;
  productId: string;
  quantity: number;
  unitPriceAmount: number;
  unitPriceCurrency: string;
  lineTotalAmount: number;
};

export type OrderDeliverySummary = {
  id: string;
  driverId: string;
  status: string;
};

export type OrderDetail = {
  id: string;
  status: string;
  commerceId: string;
  deliveryAddressId: string;
  notes?: string;
  totalAmount: number;
  totalCurrency: string;
  rejectionReason?: string;
  items: OrderItem[];
  delivery?: OrderDeliverySummary;
};

export type SaleSummary = {
  id: string;
  commerceId: string;
  driverId: string;
  status: string;
  paymentMethod: string;
  declaredPaymentMethod: string;
  declaredPaymentReceived: boolean;
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
  orderId?: string;
  status: string;
  paymentMethod: string;
  declaredPaymentMethod: string;
  declaredPaymentReceived: boolean;
  totalAmount: number;
  totalCurrency: string;
  items: SaleItem[];
};

export type CreateSaleRequest = {
  commerceId: string;
  driverId?: string;
  items: Array<{ productId: string; quantity: number }>;
  paymentMethod: string;
};

export type DeliverySummary = {
  id: string;
  orderId: string;
  driverId: string;
  status: string;
  saleId?: string;
};

export type DeliveryDetail = DeliverySummary;

export type TokenResponse = {
  accessToken: string;
  refreshToken: string;
  expiresIn: number;
};

export type UserRole = 'Admin' | 'Driver' | 'Seller' | 'CommerceContact';

export type User = {
  id: string;
  name: string;
  email: string;
  role: UserRole;
  active: boolean;
  commerceId?: string;
};

export type CreateUserRequest = {
  name: string;
  email: string;
  password: string;
  role: string;
  commerceId?: string;
};

export type DriverProfile = {
  userId: string;
  cnhNumber: string;
  cnhCategory: string;
  cnhPhotoFileId?: string;
  vehiclePlate: string;
  vehicleModel: string;
  vehicleCapacityKg?: number;
};

export type DriverProfileRequest = {
  cnhNumber: string;
  cnhCategory: string;
  cnhPhotoFileId?: string;
  vehiclePlate: string;
  vehicleModel: string;
  vehicleCapacityKg?: number;
};

export type SellerProfile = {
  userId: string;
  operatingRegion?: string;
  monthlyTargetAmount?: number;
};

export type SellerProfileRequest = {
  operatingRegion?: string;
  monthlyTargetAmount?: number;
};

export type Commerce = {
  id: string;
  cnpj: string;
  legalName: string;
  tradeName: string;
  active: boolean;
  logoFileId?: string;
};

export type CommerceSummary = Commerce;

export type RegistrationStatus = 'Active' | 'PendingReview' | 'Rejected';

export type CommerceRegistration = Commerce & {
  registrationStatus: RegistrationStatus;
  submittedByUserId?: string;
  reviewedByUserId?: string;
  rejectionReason?: string;
  registrationMode?: 'cnpj_lookup' | 'manual';
  lookupSnapshot?: Record<string, unknown>;
};

export type RejectRegistrationRequest = {
  reason: string;
};

export type PatchRegistrationRequest = {
  legalName?: string;
  tradeName?: string;
  contact?: { phone?: string; email?: string };
};

export type CreateCommerceRequest = {
  cnpj: string;
  legalName: string;
  tradeName?: string;
  address: {
    street: string;
    number: string;
    district?: string;
    city: string;
    state: string;
    postalCode: string;
  };
  contact: {
    phone?: string;
    email?: string;
  };
};

export type CommerceAddressType = 'Billing' | 'Delivery';

export type CommerceAddress = {
  id: string;
  commerceId: string;
  addressType: CommerceAddressType;
  street: string;
  number: string;
  district?: string;
  city: string;
  state: string;
  postalCode: string;
  latitude?: number;
  longitude?: number;
  isPrimary: boolean;
};

export type CommerceAddressRequest = {
  addressType: CommerceAddressType;
  street: string;
  number: string;
  district?: string;
  city: string;
  state: string;
  postalCode: string;
  isPrimary?: boolean;
};

export type UpdateCommerceAddressRequest = Partial<Omit<CommerceAddressRequest, 'addressType'>>;

export type Product = {
  id: string;
  name: string;
  sku: string;
  priceAmount: number;
  priceCurrency: string;
  active: boolean;
  categoryId?: string;
  categoryName?: string;
  categorySlug?: string;
  unitOfMeasure?: string;
  description?: string;
  isFeatured?: boolean;
};

export type ProductSummary = Pick<
  Product,
  'id' | 'name' | 'sku' | 'priceAmount' | 'priceCurrency' | 'active' | 'categoryId' | 'categoryName'
>;

export type CategorySummary = {
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

export type CategoryDetail = CategorySummary;

export type CreateCategoryRequest = {
  name: string;
  description?: string;
  sortOrder?: number;
  active?: boolean;
  slug?: string;
};

export type UpdateCategoryRequest = {
  name?: string;
  description?: string;
  sortOrder?: number;
  active?: boolean;
  slug?: string;
};

export type CreateProductRequest = {
  name: string;
  sku: string;
  priceAmount: number;
  priceCurrency?: string;
  categoryId?: string;
  unitOfMeasure?: string;
  description?: string;
};

export type UpdateProductRequest = {
  name?: string;
  priceAmount?: number;
  priceCurrency?: string;
  active?: boolean;
  categoryId?: string | null;
  unitOfMeasure?: string;
  description?: string | null;
  isFeatured?: boolean;
};

export type ProductImage = {
  id: string;
  fileId: string;
  sortOrder: number;
  isPrimary: boolean;
};

export type AttachProductImageRequest = {
  fileId: string;
  isPrimary?: boolean;
  sortOrder?: number;
};

export type StockBalance = {
  productId: string;
  available: number;
};

export type ProductStockOverview = {
  productId: string;
  sku: string;
  name: string;
  unitOfMeasure: string;
  active: boolean;
  balanceTotal: number;
  reserved: number;
  available: number;
};

export type StockMovement = {
  id: string;
  productId: string;
  responsibleId: string;
  movementType: string;
  quantity: number;
  referenceId?: string;
  reason?: string;
  createdAt: string;
};

export type RecordMovementRequest = {
  productId: string;
  movementType: 'Adjustment';
  quantity: number;
  reason: string;
};

export type ReportType = 'DailyDriver' | 'CommercePeriod' | 'Consolidated';

export type Report = {
  id: string;
  reportType: ReportType;
  periodStart: string;
  periodEnd: string;
  canonicalPayload: string;
  signature: string;
  publicKeyId: string;
  generatedAt: string;
};

export type GenerateReportRequest = {
  reportType: ReportType;
  periodStart: string;
  periodEnd: string;
  driverId?: string;
  commerceId?: string;
};

export type VerifyReportResponse = {
  valid: boolean;
  reportId: string;
};

export type AuditEvent = {
  id: string;
  actorId: string;
  action: string;
  resourceType: string;
  resourceId: string;
  metadata?: Record<string, unknown>;
  correlationId?: string;
  createdAt: string;
};

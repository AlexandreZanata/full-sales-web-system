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
};

export type CommerceSummary = Commerce;

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
  category?: string;
  unitOfMeasure?: string;
};

export type ProductSummary = Pick<
  Product,
  'id' | 'name' | 'sku' | 'priceAmount' | 'priceCurrency' | 'active'
>;

export type CreateProductRequest = {
  name: string;
  sku: string;
  priceAmount: number;
  priceCurrency?: string;
  category?: string;
  unitOfMeasure?: string;
};

export type UpdateProductRequest = {
  name?: string;
  priceAmount?: number;
  priceCurrency?: string;
  active?: boolean;
  category?: string;
  unitOfMeasure?: string;
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
};

export type StockBalance = {
  productId: string;
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

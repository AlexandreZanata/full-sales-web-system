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

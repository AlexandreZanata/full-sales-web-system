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

export type CommerceSummary = {
  id: string;
  cnpj: string;
  legalName: string;
  tradeName: string;
  active: boolean;
};

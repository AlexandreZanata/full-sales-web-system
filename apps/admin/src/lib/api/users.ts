import { apiFetch, apiPatch, apiPost, apiPut } from '@/lib/api/client';
import type {
  CreateUserRequest,
  DriverProfile,
  DriverProfileRequest,
  PaginatedResponse,
  SellerProfile,
  SellerProfileRequest,
  User,
  UserRole,
} from '@/lib/api/types';

export type UsersListParams = {
  page: number;
  pageSize: number;
  role?: UserRole | '';
};

export async function fetchUsers(params: UsersListParams): Promise<PaginatedResponse<User>> {
  if (!params.role) {
    const query = new URLSearchParams({
      page: String(params.page),
      pageSize: String(params.pageSize),
    });
    return apiFetch<PaginatedResponse<User>>(`/users?${query}`);
  }

  const batch = await apiFetch<PaginatedResponse<User>>('/users?page=1&pageSize=50');
  const filtered = batch.items.filter((user) => user.role === params.role);
  const start = (params.page - 1) * params.pageSize;
  return {
    page: params.page,
    pageSize: params.pageSize,
    total: filtered.length,
    items: filtered.slice(start, start + params.pageSize),
  };
}

export async function fetchUser(id: string): Promise<User> {
  return apiFetch<User>(`/users/${id}`);
}

export async function createUser(body: CreateUserRequest): Promise<User> {
  return apiPost<User>('/users', body);
}

export async function deactivateUser(id: string): Promise<User> {
  return apiPatch<User>(`/users/${id}/deactivate`);
}

export async function upsertDriverProfile(
  id: string,
  body: DriverProfileRequest,
): Promise<DriverProfile> {
  return apiPut<DriverProfile>(`/users/${id}/driver-profile`, body);
}

export async function upsertSellerProfile(
  id: string,
  body: SellerProfileRequest,
): Promise<SellerProfile> {
  return apiPut<SellerProfile>(`/users/${id}/seller-profile`, body);
}

export async function fetchDriversForPicker(): Promise<User[]> {
  const data = await fetchUsers({ page: 1, pageSize: 50, role: 'Driver' });
  return data.items.filter((user) => user.active);
}

import { apiFetch, apiPatch, apiPost, apiPut } from '@/lib/api/client';
import {
  type CursorListParams,
  type CursorListResponse,
  fetchAllCursorPages,
} from '@/lib/cursorPagination';
import type {
  CreateUserRequest,
  DriverProfile,
  DriverProfileRequest,
  User,
  UserRole,
} from '@/lib/api/types';

export type UsersListParams = CursorListParams & {
  role?: UserRole | '';
  active?: 'true' | 'false' | '';
};

function buildUsersQuery(params: UsersListParams): string {
  const query = new URLSearchParams();
  query.set('limit', String(params.limit ?? 20));
  if (params.cursor) {
    query.set('cursor', params.cursor);
  }
  if (params.role) {
    query.set('filter[role]', params.role);
  }
  if (params.active === 'true' || params.active === 'false') {
    query.set('filter[active]', params.active);
  }
  return query.toString();
}

export async function fetchUsers(params: UsersListParams): Promise<CursorListResponse<User>> {
  return apiFetch<CursorListResponse<User>>(`/users?${buildUsersQuery(params)}`);
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

export async function reactivateUser(id: string): Promise<User> {
  return apiPatch<User>(`/users/${id}/reactivate`);
}

export async function upsertDriverProfile(
  id: string,
  body: DriverProfileRequest,
): Promise<DriverProfile> {
  return apiPut<DriverProfile>(`/users/${id}/driver-profile`, body);
}

export { fetchSellerProfile, upsertSellerProfile } from '@/lib/api/sellerProfile';

export async function fetchDriversForPicker(): Promise<User[]> {
  const data = await fetchAllCursorPages(async (cursor) =>
    fetchUsers({ limit: 100, cursor, role: 'Driver', active: 'true' }),
  );
  return data;
}

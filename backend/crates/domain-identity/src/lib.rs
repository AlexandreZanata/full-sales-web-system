//! Identity & Access domain — User aggregate and value objects.

pub mod commerce_scope;
pub mod driver_profile;
pub mod email;
pub mod error;
pub mod full_name;
pub mod impersonation_grant;
pub mod platform_permissions;
pub mod platform_user;
pub mod platform_user_id;
pub mod role;
pub mod seller_profile;
pub mod user;
pub mod user_id;

pub use commerce_scope::{ensure_same_commerce, validate_commerce_scope};
pub use driver_profile::{DriverProfile, DriverProfileInput};
pub use email::Email;
pub use error::IdentityError;
pub use full_name::FullName;
pub use impersonation_grant::ImpersonationGrant;
pub use platform_permissions::PlatformAdminPermissions;
pub use platform_user::PlatformUser;
pub use platform_user_id::PlatformUserId;
pub use role::Role;
pub use seller_profile::{SellerProfile, SellerProfileInput};
pub use user::{RegisterUserInput, User};
pub use user_id::UserId;

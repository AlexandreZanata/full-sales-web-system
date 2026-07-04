//! Identity & Access domain — User aggregate and value objects.

pub mod email;
pub mod error;
pub mod full_name;
pub mod role;
pub mod user;
pub mod user_id;

pub use email::Email;
pub use error::IdentityError;
pub use full_name::FullName;
pub use role::Role;
pub use user::{RegisterUserInput, User};
pub use user_id::UserId;

use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum IdentityError {
    #[error("invalid email address")]
    InvalidEmail,

    #[error("full name must contain at least two parts")]
    InvalidFullName,

    #[error("invalid user id")]
    InvalidUserId,

    #[error("invalid role")]
    InvalidRole,

    #[error("user is inactive")]
    InactiveUser,

    #[error("commerce id required for CommerceContact")]
    CommerceRequired,

    #[error("commerce id must be null for this role")]
    InvalidCommerceScope,

    #[error("invalid profile field")]
    InvalidProfileField,

    #[error("forbidden")]
    Forbidden,
}

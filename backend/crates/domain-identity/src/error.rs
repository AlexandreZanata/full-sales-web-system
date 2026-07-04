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

    #[error("forbidden")]
    Forbidden,
}

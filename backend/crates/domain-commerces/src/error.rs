use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum CommerceError {
    #[error("invalid CNPJ check digits")]
    InvalidCnpj,

    #[error("invalid commerce id")]
    InvalidCommerceId,
}

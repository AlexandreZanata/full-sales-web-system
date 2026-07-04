use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum CommerceError {
    #[error("invalid CNPJ check digits")]
    InvalidCnpj,

    #[error("invalid commerce id")]
    InvalidCommerceId,

    #[error("invalid commerce address id")]
    InvalidCommerceAddressId,

    #[error("invalid address type")]
    InvalidAddressType,

    #[error("invalid postal code")]
    InvalidPostalCode,

    #[error("invalid Brazilian state (UF)")]
    InvalidState,

    #[error("invalid address field")]
    InvalidAddressField,

    #[error("duplicate primary address for commerce and type")]
    DuplicatePrimaryAddress,

    #[error("address does not belong to commerce")]
    AddressCommerceMismatch,

    #[error("address does not belong to tenant")]
    AddressTenantMismatch,

    #[error("delivery address required for order")]
    InvalidDeliveryAddress,

    #[error("inactive commerce cannot accept new sales or order addresses")]
    InactiveCommerce,

    #[error("inactive commerce cannot add delivery addresses")]
    InactiveCommerceCannotAddDeliveryAddress,
}

/// Known audit actions — free-form strings still allowed at persistence boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuditAction {
    TenantSuspend,
    TenantReactivate,
    TenantOffboard,
    TenantCreate,
    TenantPatch,
    UserDisable,
    UserEnable,
    UserPatch,
    UserResetPassword,
    ImpersonateStart,
    ImpersonateEnd,
    TenantFeaturesPatch,
    MaintenanceSchedule,
    DomainForceVerify,
    DomainPatch,
    FraudResolve,
    PaymentSettingsUpdate,
    AsaasConnect,
    AsaasDisconnect,
    SupportOrdersList,
    SupportSalesList,
    SupportProductsList,
}

impl AuditAction {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::TenantSuspend => "tenant.suspend",
            Self::TenantReactivate => "tenant.reactivate",
            Self::TenantOffboard => "tenant.offboard",
            Self::TenantCreate => "tenant.create",
            Self::TenantPatch => "tenant.patch",
            Self::UserDisable => "user.disable",
            Self::UserEnable => "user.enable",
            Self::UserPatch => "user.patch",
            Self::UserResetPassword => "user.reset_password",
            Self::ImpersonateStart => "impersonate.start",
            Self::ImpersonateEnd => "impersonate.end",
            Self::TenantFeaturesPatch => "tenant.features.patch",
            Self::MaintenanceSchedule => "maintenance.schedule",
            Self::DomainForceVerify => "domain.force_verify",
            Self::DomainPatch => "domain.patch",
            Self::FraudResolve => "fraud.resolve",
            Self::PaymentSettingsUpdate => "tenant.payment_settings.update",
            Self::AsaasConnect => "tenant.asaas.connected",
            Self::AsaasDisconnect => "tenant.asaas.disconnected",
            Self::SupportOrdersList => "support.orders.list",
            Self::SupportSalesList => "support.sales.list",
            Self::SupportProductsList => "support.products.list",
        }
    }
}

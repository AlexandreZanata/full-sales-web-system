use domain_shared::TenantId;
use uuid::Uuid;

use crate::cnpj::Cnpj;
use crate::commerce_id::CommerceId;

/// Registered business client aggregate root (Commerces context).
#[derive(Debug, Clone)]
pub struct Commerce {
    id: CommerceId,
    cnpj: Cnpj,
    legal_name: String,
    trade_name: String,
    active: bool,
    tenant_id: TenantId,
    logo_file_id: Option<Uuid>,
}

pub struct CreateCommerceInput {
    pub id: CommerceId,
    pub cnpj: Cnpj,
    pub legal_name: String,
    pub trade_name: Option<String>,
    pub tenant_id: TenantId,
}

impl Commerce {
    pub fn create(input: CreateCommerceInput) -> Self {
        Self {
            id: input.id,
            cnpj: input.cnpj,
            legal_name: input.legal_name.trim().to_owned(),
            trade_name: input
                .trade_name
                .map(|t| t.trim().to_owned())
                .filter(|t| !t.is_empty())
                .unwrap_or_else(|| input.legal_name.trim().to_owned()),
            active: true,
            tenant_id: input.tenant_id,
            logo_file_id: None,
        }
    }

    pub fn id(&self) -> CommerceId {
        self.id
    }

    pub fn cnpj(&self) -> &Cnpj {
        &self.cnpj
    }

    pub fn legal_name(&self) -> &str {
        &self.legal_name
    }

    pub fn trade_name(&self) -> &str {
        &self.trade_name
    }

    pub fn tenant_id(&self) -> TenantId {
        self.tenant_id
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn logo_file_id(&self) -> Option<Uuid> {
        self.logo_file_id
    }

    pub fn set_logo_file_id(mut self, logo_file_id: Option<Uuid>) -> Self {
        self.logo_file_id = logo_file_id;
        self
    }

    pub fn deactivate(self) -> Self {
        Self {
            active: false,
            ..self
        }
    }
}

use domain_shared::TenantId;
use uuid::Uuid;

use crate::CommerceError;
use crate::cnpj::Cnpj;
use crate::commerce_id::CommerceId;
use crate::registration_mode::RegistrationMode;
use crate::registration_status::RegistrationStatus;

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
    registration_status: RegistrationStatus,
    submitted_by_user_id: Option<Uuid>,
    reviewed_by_user_id: Option<Uuid>,
    rejection_reason: Option<String>,
    registration_mode: Option<RegistrationMode>,
}

pub struct CreateCommerceInput {
    pub id: CommerceId,
    pub cnpj: Cnpj,
    pub legal_name: String,
    pub trade_name: Option<String>,
    pub tenant_id: TenantId,
}

pub struct SubmitCommerceRegistrationInput {
    pub id: CommerceId,
    pub cnpj: Cnpj,
    pub legal_name: String,
    pub trade_name: Option<String>,
    pub tenant_id: TenantId,
    pub submitted_by_user_id: Uuid,
    pub registration_mode: RegistrationMode,
}

impl Commerce {
    pub fn create(input: CreateCommerceInput) -> Self {
        Self::from_input(
            input,
            true,
            RegistrationStatus::Active,
            None,
            None,
            None,
            None,
        )
    }

    pub fn submit_registration(input: SubmitCommerceRegistrationInput) -> Self {
        Self::from_input(
            CreateCommerceInput {
                id: input.id,
                cnpj: input.cnpj,
                legal_name: input.legal_name,
                trade_name: input.trade_name,
                tenant_id: input.tenant_id,
            },
            true,
            RegistrationStatus::PendingReview,
            Some(input.submitted_by_user_id),
            None,
            None,
            Some(input.registration_mode),
        )
    }

    fn from_input(
        input: CreateCommerceInput,
        active: bool,
        registration_status: RegistrationStatus,
        submitted_by_user_id: Option<Uuid>,
        reviewed_by_user_id: Option<Uuid>,
        rejection_reason: Option<String>,
        registration_mode: Option<RegistrationMode>,
    ) -> Self {
        let legal = input.legal_name.trim().to_owned();
        Self {
            id: input.id,
            cnpj: input.cnpj,
            legal_name: legal.clone(),
            trade_name: input
                .trade_name
                .map(|t| t.trim().to_owned())
                .filter(|t| !t.is_empty())
                .unwrap_or(legal),
            active,
            tenant_id: input.tenant_id,
            logo_file_id: None,
            registration_status,
            submitted_by_user_id,
            reviewed_by_user_id,
            rejection_reason,
            registration_mode,
        }
    }

    pub fn approve(self, reviewed_by_user_id: Uuid) -> Result<Self, CommerceError> {
        if self.registration_status != RegistrationStatus::PendingReview {
            return Err(CommerceError::InvalidRegistrationTransition);
        }
        Ok(Self {
            active: true,
            registration_status: RegistrationStatus::Active,
            reviewed_by_user_id: Some(reviewed_by_user_id),
            rejection_reason: None,
            ..self
        })
    }

    pub fn reject(self, reviewed_by_user_id: Uuid, reason: &str) -> Result<Self, CommerceError> {
        if self.registration_status != RegistrationStatus::PendingReview {
            return Err(CommerceError::InvalidRegistrationTransition);
        }
        let trimmed = reason.trim();
        if trimmed.is_empty() {
            return Err(CommerceError::RejectionReasonRequired);
        }
        Ok(Self {
            active: false,
            registration_status: RegistrationStatus::Rejected,
            reviewed_by_user_id: Some(reviewed_by_user_id),
            rejection_reason: Some(trimmed.to_owned()),
            ..self
        })
    }

    pub fn update_pending_fields(
        self,
        legal_name: String,
        trade_name: Option<String>,
    ) -> Result<Self, CommerceError> {
        if self.registration_status != RegistrationStatus::PendingReview {
            return Err(CommerceError::RegistrationNotEditable);
        }
        let legal = legal_name.trim().to_owned();
        if legal.is_empty() {
            return Err(CommerceError::InvalidAddressField);
        }
        Ok(Self {
            legal_name: legal.clone(),
            trade_name: trade_name
                .map(|t| t.trim().to_owned())
                .filter(|t| !t.is_empty())
                .unwrap_or(legal),
            ..self
        })
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

    pub fn registration_status(&self) -> RegistrationStatus {
        self.registration_status
    }

    pub fn submitted_by_user_id(&self) -> Option<Uuid> {
        self.submitted_by_user_id
    }

    pub fn reviewed_by_user_id(&self) -> Option<Uuid> {
        self.reviewed_by_user_id
    }

    pub fn rejection_reason(&self) -> Option<&str> {
        self.rejection_reason.as_deref()
    }

    pub fn registration_mode(&self) -> Option<RegistrationMode> {
        self.registration_mode
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

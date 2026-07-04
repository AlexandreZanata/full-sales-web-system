use domain_shared::TenantId;

use crate::email::Email;
use crate::error::IdentityError;
use crate::full_name::FullName;
use crate::role::Role;
use crate::user_id::UserId;

/// System account aggregate root (Identity context).
#[derive(Debug, Clone)]
pub struct User {
    id: UserId,
    name: FullName,
    email: Email,
    role: Role,
    active: bool,
    tenant_id: TenantId,
}

pub struct RegisterUserInput {
    pub id: UserId,
    pub name: FullName,
    pub email: Email,
    pub role: Role,
    pub tenant_id: TenantId,
}

impl User {
    pub fn register(input: RegisterUserInput) -> Self {
        Self {
            id: input.id,
            name: input.name,
            email: input.email,
            role: input.role,
            active: true,
            tenant_id: input.tenant_id,
        }
    }

    pub fn id(&self) -> UserId {
        self.id
    }

    pub fn name(&self) -> &FullName {
        &self.name
    }

    pub fn email(&self) -> &Email {
        &self.email
    }

    pub fn role(&self) -> Role {
        self.role
    }

    pub fn tenant_id(&self) -> TenantId {
        self.tenant_id
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    /// BR-IA-002 — inactive users cannot authenticate.
    pub fn ensure_can_authenticate(&self) -> Result<(), IdentityError> {
        if !self.active {
            return Err(IdentityError::InactiveUser);
        }
        Ok(())
    }

    pub fn deactivate(self) -> Self {
        Self {
            active: false,
            ..self
        }
    }
}

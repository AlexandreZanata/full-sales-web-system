use infra_postgres::rls::SessionContext;

use crate::auth::AuthUser;

pub fn session_from_auth(auth: &AuthUser) -> SessionContext {
    SessionContext {
        tenant_id: auth.tenant_id,
        role: auth.role.as_str().to_owned(),
        user_id: auth.user_id,
        commerce_id: auth.commerce_id,
    }
}

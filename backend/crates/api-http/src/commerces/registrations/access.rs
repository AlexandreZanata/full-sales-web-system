use crate::auth::AuthUser;
use crate::error::ApiError;
use crate::state::AppState;

pub(crate) async fn load_review_flag(state: &AppState, auth: &AuthUser) -> Result<bool, ApiError> {
    infra_postgres::identity::user_can_review_commerce(
        &state.app_pool,
        auth.tenant_id,
        auth.user_id,
    )
    .await
    .map_err(|_| ApiError::internal())
}

pub(crate) async fn ensure_registration_readable(
    state: &AppState,
    auth: &AuthUser,
    row: &infra_postgres::commerces::CommerceRow,
) -> Result<(), ApiError> {
    let review_flag = load_review_flag(state, auth).await?;
    if application::can_review_commerce(auth.role, review_flag) {
        return Ok(());
    }
    if auth.role.can_submit_commerce() && row.submitted_by_user_id == Some(auth.user_id) {
        return Ok(());
    }
    Err(ApiError::forbidden())
}

pub(crate) async fn ensure_registration_submitter(
    state: &AppState,
    auth: &AuthUser,
    row: &infra_postgres::commerces::CommerceRow,
) -> Result<(), ApiError> {
    let review_flag = load_review_flag(state, auth).await?;
    if application::can_review_commerce(auth.role, review_flag) {
        return Ok(());
    }
    if row.submitted_by_user_id == Some(auth.user_id) && row.registration_status == "PendingReview"
    {
        return Ok(());
    }
    Err(ApiError::forbidden())
}

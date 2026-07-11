use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use domain_billing::{BillingInterval, SubscriptionStatus};
use domain_shared::TenantId;
use infra_crypto::PasswordHasher;
use serde::Deserialize;
use uuid::Uuid;

use crate::audit_context::AuditRequestContext;
use crate::error::ApiError;
use crate::platform::auth::PlatformAuthUser;
use crate::platform::tenants::types::{
    ProvisionTenantResponse, TenantDetailResponse, map_platform_patch_error, row_to_tenant,
    tenant_detail,
};
use crate::platform_audit::record_platform_audit;
use crate::state::AppState;

#[derive(Deserialize)]
pub struct CreateTenantRequest {
    #[serde(rename = "legalName")]
    pub legal_name: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "adminEmail")]
    pub admin_email: String,
    #[serde(rename = "planId")]
    pub plan_id: Uuid,
    pub trial: Option<bool>,
    #[serde(rename = "seedDemoCatalog")]
    pub seed_demo_catalog: Option<bool>,
    pub cnpj: Option<String>,
}

#[derive(Deserialize)]
pub struct PatchTenantRequest {
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
    #[serde(rename = "planId")]
    pub plan_id: Option<Uuid>,
    #[serde(rename = "graceExtendedUntil")]
    pub grace_extended_until: Option<chrono::DateTime<chrono::Utc>>,
    pub settings: Option<serde_json::Value>,
}

#[derive(Deserialize)]
pub struct SuspendRequest {
    pub reason: String,
}

pub async fn create_tenant(
    State(state): State<AppState>,
    auth: PlatformAuthUser,
    ctx: AuditRequestContext,
    Json(body): Json<CreateTenantRequest>,
) -> Result<(StatusCode, Json<ProvisionTenantResponse>), ApiError> {
    if !infra_postgres::shared::plan_exists(&state.admin_pool, body.plan_id)
        .await
        .map_err(|_| ApiError::internal())?
    {
        return Err(ApiError::bad_request(
            "PLAN_NOT_FOUND",
            "Subscription plan not found",
        ));
    }

    let admin_email = body.admin_email.trim().to_lowercase();
    crate::fraud::check_blocklist(
        &state,
        Some(&admin_email),
        body.cnpj.as_deref(),
        None,
        None,
        None,
    )
    .await?;
    crate::fraud::on_provision_attempt(&state, auth.user_id).await?;

    let tenant_id = TenantId::generate();
    let admin_user_id = Uuid::now_v7();
    let temp_password = Uuid::now_v7().to_string();
    let password_hash = PasswordHasher::hash(&temp_password).map_err(|_| ApiError::internal())?;

    let input = application::tenants::ProvisionTenantInput {
        legal_name: body.legal_name.clone(),
        display_name: body.display_name.clone(),
        admin_email: admin_email.clone(),
        plan_id: body.plan_id,
        trial: body.trial.unwrap_or(true),
        seed_demo_catalog: body.seed_demo_catalog.unwrap_or(false),
    };

    let mut tenant = application::tenants::build_staging_tenant(tenant_id, &input)
        .map_err(map_platform_patch_error)?;

    let mut settings = tenant.settings.clone();
    if input.seed_demo_catalog {
        settings["seedDemoCatalog"] = serde_json::json!(true);
    }

    infra_postgres::shared::provision_tenant(
        &state.admin_pool,
        &state.app_pool,
        infra_postgres::shared::ProvisionTenantParams {
            id: tenant_id,
            legal_name: &body.legal_name,
            display_name: &body.display_name,
            status: tenant.status,
            plan_id: tenant.plan_id,
            trial_ends_at: None,
            settings: settings.clone(),
            admin_user_id,
            admin_email: &admin_email,
            admin_name: "Tenant Admin",
            admin_password_hash: &password_hash,
        },
    )
    .await
    .map_err(|_| ApiError::internal())?;

    let cpf_cnpj = body
        .cnpj
        .clone()
        .unwrap_or_else(|| "11222333000181".to_owned());
    let customer_req = application::billing::CreateCustomerRequest {
        name: body.legal_name.clone(),
        cpf_cnpj,
        email: input.admin_email.clone(),
        external_reference: tenant_id.as_uuid().to_string(),
    };

    let mut trial_ends = None;
    match state.payment_gateway.create_customer(customer_req).await {
        Ok(customer) => {
            infra_postgres::billing::set_tenant_asaas_customer(
                &state.admin_pool,
                tenant_id,
                &customer.id,
            )
            .await
            .map_err(|_| ApiError::internal())?;

            if let Some(plan) = infra_postgres::billing::find_plan(&state.admin_pool, body.plan_id)
                .await
                .map_err(|_| ApiError::internal())?
            {
                let interval = BillingInterval::parse(&plan.billing_interval)
                    .map_err(|_| ApiError::internal())?;
                let sub_result = application::billing::provision_subscription(
                    state.payment_gateway.as_ref(),
                    &application::billing::ProvisionSubscriptionInput {
                        tenant_id,
                        plan_id: plan.id,
                        customer_id: customer.id.clone(),
                        plan_name: plan.name.clone(),
                        price_minor: plan.price_minor,
                        billing_interval: interval,
                    },
                )
                .await
                .map_err(|_| ApiError::internal())?;

                infra_postgres::billing::insert_subscription(
                    &state.admin_pool,
                    infra_postgres::billing::SubscriptionInsert {
                        id: sub_result.subscription_id,
                        tenant_id,
                        plan_id: plan.id,
                        asaas_subscription_id: Some(sub_result.asaas_subscription_id),
                        status: SubscriptionStatus::Pending,
                        current_period_end: None,
                    },
                )
                .await
                .map_err(|_| ApiError::internal())?;
            }

            trial_ends = application::tenants::finalize_provision_tenant(&mut tenant, &input)
                .map_err(map_platform_patch_error)?;
            infra_postgres::shared::update_tenant_lifecycle(
                &state.admin_pool,
                tenant_id,
                tenant.status,
                tenant.plan_id,
                tenant.trial_ends_at,
                None,
                None,
                None,
                None,
                Some(settings),
            )
            .await
            .map_err(|_| ApiError::internal())?;
        }
        Err(err) => {
            let code = application::billing::map_billing_error(err);
            infra_postgres::billing::insert_provisioning_dead_letter(
                &state.admin_pool,
                tenant_id,
                &code,
                "Asaas customer provisioning failed",
                serde_json::json!({ "adminEmail": input.admin_email }),
            )
            .await
            .map_err(|_| ApiError::internal())?;
        }
    }

    record_platform_audit(
        &state,
        &ctx,
        auth.user_id,
        "tenant.create",
        Some(tenant_id),
        "Tenant",
        tenant_id.as_uuid(),
        None,
    )
    .await?;

    Ok((
        StatusCode::CREATED,
        Json(ProvisionTenantResponse {
            tenant_id: tenant_id.as_uuid(),
            admin_user_id,
            status: tenant.status.as_str().to_owned(),
            trial_ends_at: trial_ends,
            admin_temporary_password: temp_password,
        }),
    ))
}

pub async fn get_tenant(
    State(state): State<AppState>,
    _auth: PlatformAuthUser,
    Path(id): Path<Uuid>,
) -> Result<Json<TenantDetailResponse>, ApiError> {
    let tenant_id = TenantId::from_uuid(id);
    let row = infra_postgres::shared::find_tenant_lifecycle(&state.admin_pool, tenant_id)
        .await
        .map_err(|_| ApiError::internal())?
        .ok_or_else(ApiError::not_found)?;
    let counts = infra_postgres::shared::tenant_counts(&state.app_pool, tenant_id)
        .await
        .map_err(|_| ApiError::internal())?;
    Ok(Json(tenant_detail(row, counts)))
}

pub async fn patch_tenant(
    State(state): State<AppState>,
    auth: PlatformAuthUser,
    ctx: AuditRequestContext,
    Path(id): Path<Uuid>,
    Json(body): Json<PatchTenantRequest>,
) -> Result<Json<TenantDetailResponse>, ApiError> {
    let tenant_id = TenantId::from_uuid(id);
    let row = load_row(&state, tenant_id).await?;
    let tenant = row_to_tenant(&row);

    if let Some(plan_id) = body.plan_id {
        crate::billing::change_tenant_plan(&state.admin_pool, tenant_id, plan_id)
            .await
            .map_err(|_| ApiError::bad_request("PLAN_NOT_FOUND", "Subscription plan not found"))?;
    }
    if let Some(until) = body.grace_extended_until {
        infra_postgres::shared::set_grace_extended_until(&state.admin_pool, tenant_id, until)
            .await
            .map_err(|_| ApiError::internal())?;
    }

    let plan_id = body.plan_id.or(tenant.plan_id);
    let updated = infra_postgres::shared::update_tenant_lifecycle(
        &state.admin_pool,
        tenant_id,
        tenant.status,
        plan_id,
        tenant.trial_ends_at,
        tenant.suspended_at,
        tenant.suspended_reason.as_deref(),
        tenant.offboarding_scheduled_at,
        body.display_name.as_deref(),
        body.settings.clone(),
    )
    .await
    .map_err(|_| ApiError::internal())?;
    if !updated {
        return Err(ApiError::not_found());
    }
    record_platform_audit(
        &state,
        &ctx,
        auth.user_id,
        "tenant.patch",
        Some(tenant_id),
        "Tenant",
        id,
        None,
    )
    .await?;
    get_tenant(State(state), auth, Path(id)).await
}

pub async fn suspend_tenant(
    State(state): State<AppState>,
    auth: PlatformAuthUser,
    ctx: AuditRequestContext,
    Path(id): Path<Uuid>,
    Json(body): Json<SuspendRequest>,
) -> Result<Json<TenantDetailResponse>, ApiError> {
    let tenant_id = TenantId::from_uuid(id);
    let response = persist_transition(&state, tenant_id, |tenant| {
        application::tenants::apply_suspend(tenant, &body.reason)
    })
    .await?;
    record_platform_audit(
        &state,
        &ctx,
        auth.user_id,
        "tenant.suspend",
        Some(tenant_id),
        "Tenant",
        id,
        Some(serde_json::json!({ "reason": body.reason })),
    )
    .await?;
    Ok(response)
}

pub async fn reactivate_tenant(
    State(state): State<AppState>,
    auth: PlatformAuthUser,
    ctx: AuditRequestContext,
    Path(id): Path<Uuid>,
) -> Result<Json<TenantDetailResponse>, ApiError> {
    let tenant_id = TenantId::from_uuid(id);
    let response =
        persist_transition(&state, tenant_id, application::tenants::apply_reactivate).await?;
    record_platform_audit(
        &state,
        &ctx,
        auth.user_id,
        "tenant.reactivate",
        Some(tenant_id),
        "Tenant",
        id,
        None,
    )
    .await?;
    Ok(response)
}

pub async fn offboard_tenant(
    State(state): State<AppState>,
    auth: PlatformAuthUser,
    ctx: AuditRequestContext,
    Path(id): Path<Uuid>,
) -> Result<Json<TenantDetailResponse>, ApiError> {
    let tenant_id = TenantId::from_uuid(id);
    let response =
        persist_transition(&state, tenant_id, application::tenants::apply_offboard).await?;
    record_platform_audit(
        &state,
        &ctx,
        auth.user_id,
        "tenant.offboard",
        Some(tenant_id),
        "Tenant",
        id,
        None,
    )
    .await?;
    Ok(response)
}

pub async fn run_offboarding_job(
    State(state): State<AppState>,
    _auth: PlatformAuthUser,
) -> Result<Json<serde_json::Value>, ApiError> {
    let candidates = infra_postgres::shared::find_offboarding_candidates(
        &state.admin_pool,
        application::tenants::RETENTION_DAYS,
    )
    .await
    .map_err(|_| ApiError::internal())?;
    let mut processed = Vec::new();
    let mut skipped = Vec::new();
    for tenant_id in candidates {
        let anonymized = infra_postgres::shared::anonymize_tenant_pii(&state.admin_pool, tenant_id)
            .await
            .map_err(|_| ApiError::internal())?;
        if anonymized {
            processed.push(tenant_id.as_uuid());
        } else {
            skipped.push(tenant_id.as_uuid());
        }
    }
    Ok(Json(
        serde_json::json!({ "processed": processed, "skipped": skipped }),
    ))
}

pub async fn run_dunning_job(
    State(state): State<AppState>,
    _auth: PlatformAuthUser,
) -> Result<Json<serde_json::Value>, ApiError> {
    let processed = crate::billing::run_dunning_job(&state.admin_pool)
        .await
        .map_err(|_| ApiError::internal())?;
    Ok(Json(
        serde_json::json!({ "processed": processed, "emailNotifications": "stub" }),
    ))
}

async fn persist_transition(
    state: &AppState,
    tenant_id: TenantId,
    apply: impl FnOnce(&mut domain_platform::Tenant) -> Result<(), application::AppError>,
) -> Result<Json<TenantDetailResponse>, ApiError> {
    let row = load_row(state, tenant_id).await?;
    let mut tenant = row_to_tenant(&row);
    apply(&mut tenant).map_err(map_platform_patch_error)?;
    save_tenant(state, &tenant).await?;
    let counts = infra_postgres::shared::tenant_counts(&state.app_pool, tenant_id)
        .await
        .map_err(|_| ApiError::internal())?;
    Ok(Json(tenant_detail(
        infra_postgres::shared::find_tenant_lifecycle(&state.admin_pool, tenant_id)
            .await
            .map_err(|_| ApiError::internal())?
            .expect("tenant"),
        counts,
    )))
}

async fn load_row(
    state: &AppState,
    tenant_id: TenantId,
) -> Result<infra_postgres::shared::TenantLifecycleRow, ApiError> {
    infra_postgres::shared::find_tenant_lifecycle(&state.admin_pool, tenant_id)
        .await
        .map_err(|_| ApiError::internal())?
        .ok_or_else(ApiError::not_found)
}

async fn save_tenant(state: &AppState, tenant: &domain_platform::Tenant) -> Result<(), ApiError> {
    infra_postgres::shared::update_tenant_lifecycle(
        &state.admin_pool,
        tenant.id,
        tenant.status,
        tenant.plan_id,
        tenant.trial_ends_at,
        tenant.suspended_at,
        tenant.suspended_reason.as_deref(),
        tenant.offboarding_scheduled_at,
        None,
        None,
    )
    .await
    .map_err(|_| ApiError::internal())?;
    Ok(())
}

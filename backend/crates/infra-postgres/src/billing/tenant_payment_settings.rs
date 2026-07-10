use domain_billing::PaymentMethodToggles;
use domain_shared::TenantId;
use sqlx::PgPool;

use crate::PostgresError;
use crate::rls::apply_tenant_context;

#[derive(Debug, Clone)]
pub struct PaymentSettingsRow {
    pub enabled: bool,
    pub methods: PaymentMethodToggles,
    pub auto_capture: bool,
}

pub async fn find_payment_settings(
    pool: &PgPool,
    tenant_id: TenantId,
) -> Result<Option<PaymentSettingsRow>, PostgresError> {
    let row = sqlx::query_as::<_, (bool, bool, bool, bool, bool)>(
        "SELECT enabled, method_pix, method_credit, method_boleto, auto_capture
         FROM billing.tenant_payment_settings
         WHERE tenant_id = $1",
    )
    .bind(tenant_id.as_uuid())
    .fetch_optional(pool)
    .await?;

    Ok(row.map(
        |(enabled, pix, credit, boleto, auto_capture)| PaymentSettingsRow {
            enabled,
            methods: PaymentMethodToggles {
                pix,
                credit,
                boleto,
            },
            auto_capture,
        },
    ))
}

pub async fn upsert_payment_settings(
    pool: &PgPool,
    tenant_id: TenantId,
    enabled: bool,
    methods: PaymentMethodToggles,
    auto_capture: bool,
) -> Result<(), PostgresError> {
    sqlx::query(
        "INSERT INTO billing.tenant_payment_settings
         (tenant_id, enabled, method_pix, method_credit, method_boleto, auto_capture)
         VALUES ($1, $2, $3, $4, $5, $6)
         ON CONFLICT (tenant_id) DO UPDATE SET
           enabled = EXCLUDED.enabled,
           method_pix = EXCLUDED.method_pix,
           method_credit = EXCLUDED.method_credit,
           method_boleto = EXCLUDED.method_boleto,
           auto_capture = EXCLUDED.auto_capture,
           updated_at = now()",
    )
    .bind(tenant_id.as_uuid())
    .bind(enabled)
    .bind(methods.pix)
    .bind(methods.credit)
    .bind(methods.boleto)
    .bind(auto_capture)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn upsert_payment_settings_app(
    pool: &PgPool,
    tenant_id: TenantId,
    enabled: bool,
    methods: PaymentMethodToggles,
    auto_capture: bool,
) -> Result<(), PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    sqlx::query(
        "INSERT INTO billing.tenant_payment_settings
         (tenant_id, enabled, method_pix, method_credit, method_boleto, auto_capture)
         VALUES ($1, $2, $3, $4, $5, $6)
         ON CONFLICT (tenant_id) DO UPDATE SET
           enabled = EXCLUDED.enabled,
           method_pix = EXCLUDED.method_pix,
           method_credit = EXCLUDED.method_credit,
           method_boleto = EXCLUDED.method_boleto,
           auto_capture = EXCLUDED.auto_capture,
           updated_at = now()",
    )
    .bind(tenant_id.as_uuid())
    .bind(enabled)
    .bind(methods.pix)
    .bind(methods.credit)
    .bind(methods.boleto)
    .bind(auto_capture)
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(())
}

pub async fn disable_online_payments(
    pool: &PgPool,
    tenant_id: TenantId,
) -> Result<(), PostgresError> {
    sqlx::query(
        "UPDATE billing.tenant_payment_settings SET enabled = false, updated_at = now()
         WHERE tenant_id = $1",
    )
    .bind(tenant_id.as_uuid())
    .execute(pool)
    .await?;
    Ok(())
}

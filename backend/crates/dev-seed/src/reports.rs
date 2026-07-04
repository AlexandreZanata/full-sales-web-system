use chrono::{TimeZone, Utc};
use domain_commerces::CommerceId;
use domain_identity::UserId;
use domain_reports::{ReportAssemblyInput, ReportPeriod, ReportSaleFact, sign_canonical_payload};
use domain_sales::{DeclaredPaymentMethod, SaleId, SaleStatus};
use domain_shared::TenantId;
use ed25519_dalek::SigningKey;
use infra_postgres::PgPool;
use infra_postgres::reports::{self, NewReport};
use uuid::Uuid;

use crate::commerces::CommercesSeed;
use crate::error::DevSeedResult;
use crate::foundation::FoundationSeed;
use crate::ids::{DEV_SIGNING_KEY_ID, report_ids};
use crate::users::UsersSeed;

pub async fn seed_reports(
    app_pool: &PgPool,
    foundation: &FoundationSeed,
    users: &UsersSeed,
    commerces: &CommercesSeed,
) -> DevSeedResult<()> {
    let tenant = foundation.tenant_id;
    let ids = report_ids();

    insert_driver_report(
        app_pool,
        tenant,
        &foundation.signing_key,
        ids[0],
        users.driver_a_id,
        commerces.commerce_a_id,
        Utc.with_ymd_and_hms(2026, 1, 1, 0, 0, 0).unwrap(),
        Utc.with_ymd_and_hms(2026, 12, 31, 23, 59, 59).unwrap(),
        vec![sample_sale_fact(commerces.commerce_a_id)],
    )
    .await?;

    insert_driver_report(
        app_pool,
        tenant,
        &foundation.signing_key,
        ids[1],
        users.driver_b_id,
        commerces.commerce_b_id,
        Utc.with_ymd_and_hms(2099, 1, 1, 0, 0, 0).unwrap(),
        Utc.with_ymd_and_hms(2099, 1, 31, 23, 59, 59).unwrap(),
        vec![],
    )
    .await?;

    Ok(())
}

fn sample_sale_fact(commerce_id: Uuid) -> ReportSaleFact {
    ReportSaleFact {
        sale_id: SaleId::from_uuid(
            Uuid::parse_str("01900001-0050-7000-8000-000000000002").expect("sale"),
        ),
        order_id: None,
        commerce_id: CommerceId::from_uuid(commerce_id),
        amount_cents: 2_500_00,
        currency: "BRL".into(),
        sale_status: SaleStatus::Confirmed,
        order_status: None,
        declared_method: DeclaredPaymentMethod::Cash,
        declared_received: true,
    }
}

async fn insert_driver_report(
    app_pool: &PgPool,
    tenant: TenantId,
    signing_key: &SigningKey,
    report_id: Uuid,
    driver_id: Uuid,
    _commerce_id: Uuid,
    period_start: chrono::DateTime<Utc>,
    period_end: chrono::DateTime<Utc>,
    sales: Vec<ReportSaleFact>,
) -> DevSeedResult<()> {
    if reports::find_report_by_id(app_pool, tenant, report_id)
        .await?
        .is_some()
    {
        return Ok(());
    }
    let assembled = ReportAssemblyInput {
        period: ReportPeriod {
            start: period_start,
            end: period_end,
        },
        driver_id: UserId::from_uuid(driver_id),
        sales,
    }
    .assemble()
    .map_err(|err| crate::error::DevSeedError::Aborted(err.to_string()))?;
    let signature = sign_canonical_payload(&assembled.canonical_json, signing_key);
    reports::insert_report(
        app_pool,
        tenant,
        NewReport {
            id: report_id,
            report_type: "DailyDriver",
            period_start,
            period_end,
            canonical_payload: &assembled.canonical_json,
            signature: &signature,
            public_key_id: DEV_SIGNING_KEY_ID,
        },
    )
    .await?;
    Ok(())
}

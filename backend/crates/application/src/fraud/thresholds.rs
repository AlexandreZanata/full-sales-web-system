use std::time::Duration;

use serde::Deserialize;

#[derive(Debug, Clone, Copy)]
pub struct FraudThresholds {
    pub login_failure_max: u32,
    pub login_failure_window: Duration,
    pub payment_velocity_max: u32,
    pub payment_velocity_window: Duration,
    pub provision_alert_max: u32,
    pub provision_alert_window: Duration,
    pub webhook_failure_burst_max: u32,
    pub webhook_failure_burst_window: Duration,
    pub tenant_fraud_score_block: i32,
}

#[derive(Debug, Deserialize)]
struct RawThresholds {
    #[serde(rename = "loginFailureMax", default = "default_login_failure_max")]
    login_failure_max: u32,
    #[serde(rename = "loginFailureWindowSecs", default = "default_login_failure_window_secs")]
    login_failure_window_secs: u64,
    #[serde(rename = "paymentVelocityMax", default = "default_payment_velocity_max")]
    payment_velocity_max: u32,
    #[serde(rename = "paymentVelocityWindowSecs", default = "default_payment_velocity_window_secs")]
    payment_velocity_window_secs: u64,
    #[serde(rename = "provisionAlertMax", default = "default_provision_alert_max")]
    provision_alert_max: u32,
    #[serde(rename = "provisionAlertWindowSecs", default = "default_provision_alert_window_secs")]
    provision_alert_window_secs: u64,
    #[serde(rename = "webhookFailureBurstMax", default = "default_webhook_failure_burst_max")]
    webhook_failure_burst_max: u32,
    #[serde(
        rename = "webhookFailureBurstWindowSecs",
        default = "default_webhook_failure_burst_window_secs"
    )]
    webhook_failure_burst_window_secs: u64,
    #[serde(rename = "tenantFraudScoreBlock", default = "default_tenant_fraud_score_block")]
    tenant_fraud_score_block: i32,
}

impl FraudThresholds {
    pub fn defaults() -> Self {
        Self::from_json(&serde_json::json!({}))
    }

    pub fn from_json(value: &serde_json::Value) -> Self {
        let raw: RawThresholds = serde_json::from_value(value.clone()).unwrap_or(RawThresholds {
            login_failure_max: default_login_failure_max(),
            login_failure_window_secs: default_login_failure_window_secs(),
            payment_velocity_max: default_payment_velocity_max(),
            payment_velocity_window_secs: default_payment_velocity_window_secs(),
            provision_alert_max: default_provision_alert_max(),
            provision_alert_window_secs: default_provision_alert_window_secs(),
            webhook_failure_burst_max: default_webhook_failure_burst_max(),
            webhook_failure_burst_window_secs: default_webhook_failure_burst_window_secs(),
            tenant_fraud_score_block: default_tenant_fraud_score_block(),
        });
        Self {
            login_failure_max: raw.login_failure_max,
            login_failure_window: Duration::from_secs(raw.login_failure_window_secs),
            payment_velocity_max: raw.payment_velocity_max,
            payment_velocity_window: Duration::from_secs(raw.payment_velocity_window_secs),
            provision_alert_max: raw.provision_alert_max,
            provision_alert_window: Duration::from_secs(raw.provision_alert_window_secs),
            webhook_failure_burst_max: raw.webhook_failure_burst_max,
            webhook_failure_burst_window: Duration::from_secs(raw.webhook_failure_burst_window_secs),
            tenant_fraud_score_block: raw.tenant_fraud_score_block,
        }
    }

    pub fn checkout_blocked(&self, score: i32) -> bool {
        score >= self.tenant_fraud_score_block
    }
}

fn default_login_failure_max() -> u32 {
    5
}
fn default_login_failure_window_secs() -> u64 {
    3600
}
fn default_payment_velocity_max() -> u32 {
    20
}
fn default_payment_velocity_window_secs() -> u64 {
    3600
}
fn default_provision_alert_max() -> u32 {
    10
}
fn default_provision_alert_window_secs() -> u64 {
    3600
}
fn default_webhook_failure_burst_max() -> u32 {
    10
}
fn default_webhook_failure_burst_window_secs() -> u64 {
    300
}
fn default_tenant_fraud_score_block() -> i32 {
    100
}

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PaymentMethodsResponse {
    pub pix: bool,
    pub credit: bool,
    pub boleto: bool,
}

#[derive(Serialize)]
pub struct AsaasConnectionResponse {
    pub connected: bool,
    #[serde(rename = "apiKeyLast4", skip_serializing_if = "Option::is_none")]
    pub api_key_last4: Option<String>,
    #[serde(rename = "connectedAt", skip_serializing_if = "Option::is_none")]
    pub connected_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Serialize)]
pub struct PaymentSettingsResponse {
    pub enabled: bool,
    pub methods: PaymentMethodsResponse,
    #[serde(rename = "autoCapture")]
    pub auto_capture: bool,
    pub asaas: AsaasConnectionResponse,
}

#[derive(Deserialize)]
pub struct UpdatePaymentSettingsRequest {
    pub enabled: bool,
    pub methods: PaymentMethodsResponse,
    #[serde(rename = "autoCapture")]
    pub auto_capture: bool,
}

#[derive(Deserialize)]
pub struct ConnectAsaasRequest {
    #[serde(rename = "apiKey")]
    pub api_key: String,
}

#[derive(Serialize)]
pub struct ConnectAsaasResponse {
    pub connected: bool,
    #[serde(rename = "accountName")]
    pub account_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct PaymentBalanceResponse {
    #[serde(rename = "balanceMinor")]
    pub balance_minor: i64,
    pub currency: String,
}

#[derive(Deserialize)]
pub struct TransactionsQuery {
    pub offset: Option<u32>,
    pub limit: Option<u32>,
}

#[derive(Serialize, Deserialize)]
pub struct TransactionItem {
    pub id: String,
    #[serde(rename = "type")]
    pub tx_type: String,
    #[serde(rename = "amountMinor")]
    pub amount_minor: i64,
    pub date: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct TransactionsResponse {
    pub data: Vec<TransactionItem>,
    #[serde(rename = "hasMore")]
    pub has_more: bool,
}

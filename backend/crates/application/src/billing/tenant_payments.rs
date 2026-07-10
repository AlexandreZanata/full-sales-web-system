use domain_billing::PaymentMethodToggles;

pub const ORDER_PAYMENT_REFERENCE_PREFIX: &str = "order:";

pub fn order_payment_external_reference(order_id: uuid::Uuid) -> String {
    format!("{ORDER_PAYMENT_REFERENCE_PREFIX}{order_id}")
}

pub fn parse_order_payment_reference(reference: &str) -> Option<uuid::Uuid> {
    reference
        .strip_prefix(ORDER_PAYMENT_REFERENCE_PREFIX)
        .and_then(|id| uuid::Uuid::parse_str(id).ok())
}

pub fn api_key_last4(api_key: &str) -> Option<String> {
    let trimmed = api_key.trim();
    if trimmed.len() < 4 {
        return None;
    }
    let last4: String = trimmed
        .chars()
        .rev()
        .take(4)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect();
    Some(last4)
}

pub fn primary_billing_type(methods: PaymentMethodToggles) -> &'static str {
    if methods.pix {
        "PIX"
    } else if methods.credit {
        "CREDIT_CARD"
    } else {
        "BOLETO"
    }
}

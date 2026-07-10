//! Optional live Asaas sandbox test — run with `ASAAS_SANDBOX=1 cargo test platform_saas_sandbox -- --ignored`.

use crate::support::setup;

#[tokio::test]
#[ignore = "requires ASAAS_SANDBOX=1 and live Asaas credentials in env"]
async fn platform_saas_sandbox_when_env_set_then_smoke() {
    if std::env::var("ASAAS_SANDBOX").ok().as_deref() != Some("1") {
        panic!("Set ASAAS_SANDBOX=1 to run sandbox smoke test");
    }
    let _env = setup().await;
    assert!(
        std::env::var("ASAAS_API_KEY").is_ok(),
        "ASAAS_API_KEY required for sandbox smoke"
    );
}

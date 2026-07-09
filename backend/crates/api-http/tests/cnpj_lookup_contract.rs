//! Contract tests: CNPJ lookup handler (Phase 70 manual validation checklist).

mod support;

use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

use api_http::cnpj_lookup::{CnpjLookupError, CnpjLookupProvider, CnpjLookupResult};
use async_trait::async_trait;
use http::StatusCode;
use infra_redis::RateLimitPolicy;
use support::{request, seed_seller, setup};

struct UnavailableCnpjLookup;

#[async_trait]
impl CnpjLookupProvider for UnavailableCnpjLookup {
    async fn lookup(&self, _cnpj: &str) -> Result<CnpjLookupResult, CnpjLookupError> {
        Err(CnpjLookupError::Unavailable)
    }
}

struct CountingCnpjLookup {
    calls: Arc<AtomicUsize>,
}

#[async_trait]
impl CnpjLookupProvider for CountingCnpjLookup {
    async fn lookup(&self, cnpj: &str) -> Result<CnpjLookupResult, CnpjLookupError> {
        self.calls.fetch_add(1, Ordering::SeqCst);
        if cnpj == "11222333000181" {
            Ok(CnpjLookupResult {
                cnpj: cnpj.to_owned(),
                legal_name: "Acme Comercio Ltda".into(),
                trade_name: "Acme Store".into(),
                address: api_http::cnpj_lookup::CnpjLookupAddress {
                    street: "Rua Example".into(),
                    number: "100".into(),
                    district: "Centro".into(),
                    city: "São Paulo".into(),
                    state: "SP".into(),
                    postal_code: "01001000".into(),
                },
                provider: "mock".into(),
                fetched_at: chrono::Utc::now(),
            })
        } else {
            Err(CnpjLookupError::NotFound)
        }
    }
}

#[tokio::test]
async fn contract_cnpj_lookup_when_valid_known_cnpj_then_200() {
    let env = setup().await;
    let (_seller_id, token) = seed_seller(&env, "seller-cnpj-valid@test.com").await;

    let (status, body) = request(
        &env,
        "GET",
        "/v1/commerces/cnpj-lookup?cnpj=11222333000181",
        Some(&token),
        None,
    )
    .await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["legalName"], "Acme Comercio Ltda");
    assert_eq!(body["provider"], "mock");
}

#[tokio::test]
async fn contract_cnpj_lookup_when_unknown_cnpj_then_not_found() {
    let env = setup().await;
    let (_seller_id, token) = seed_seller(&env, "seller-cnpj-unknown@test.com").await;

    let (status, body) = request(
        &env,
        "GET",
        "/v1/commerces/cnpj-lookup?cnpj=99999999000191",
        Some(&token),
        None,
    )
    .await;

    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(body["error"]["code"], "CNPJ_NOT_FOUND");
}

#[tokio::test]
async fn contract_cnpj_lookup_when_invalid_check_digits_then_bad_request() {
    let env = setup().await;
    let (_seller_id, token) = seed_seller(&env, "seller-cnpj-invalid@test.com").await;

    let (status, body) = request(
        &env,
        "GET",
        "/v1/commerces/cnpj-lookup?cnpj=11222333000180",
        Some(&token),
        None,
    )
    .await;

    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(body["error"]["code"], "INVALID_CNPJ");
}

#[tokio::test]
async fn contract_cnpj_lookup_when_upstream_unavailable_then_502() {
    let mut env = setup().await;
    env.state.cnpj_lookup = Arc::new(UnavailableCnpjLookup);
    let (_seller_id, token) = seed_seller(&env, "seller-cnpj-upstream@test.com").await;

    let (status, body) = request(
        &env,
        "GET",
        "/v1/commerces/cnpj-lookup?cnpj=11222333000181",
        Some(&token),
        None,
    )
    .await;

    assert_eq!(status, StatusCode::BAD_GATEWAY);
    assert_eq!(body["error"]["code"], "CNPJ_LOOKUP_UNAVAILABLE");
}

#[tokio::test]
async fn contract_cnpj_lookup_when_burst_then_rate_limited() {
    let mut env = setup().await;
    env.state.cnpj_lookup_rate_limit = RateLimitPolicy {
        max: 2,
        window: Duration::from_secs(60),
    };
    let (_seller_id, token) = seed_seller(&env, "seller-cnpj-ratelimit@test.com").await;
    let path = "/v1/commerces/cnpj-lookup?cnpj=11222333000181";

    for _ in 0..2 {
        let (status, _) = request(&env, "GET", path, Some(&token), None).await;
        assert_eq!(status, StatusCode::OK);
    }

    let (status, body) = request(&env, "GET", path, Some(&token), None).await;
    assert_eq!(status, StatusCode::TOO_MANY_REQUESTS);
    assert_eq!(body["error"]["code"], "RATE_LIMITED");
}

#[tokio::test]
async fn contract_cnpj_lookup_when_cached_miss_then_skips_upstream() {
    let mut env = setup().await;
    let calls = Arc::new(AtomicUsize::new(0));
    env.state.cnpj_lookup = Arc::new(CountingCnpjLookup {
        calls: calls.clone(),
    });
    let (_seller_id, token) = seed_seller(&env, "seller-cnpj-cache@test.com").await;
    let path = "/v1/commerces/cnpj-lookup?cnpj=99999999000191";

    let (status, _) = request(&env, "GET", path, Some(&token), None).await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(calls.load(Ordering::SeqCst), 1);

    let (status, body) = request(&env, "GET", path, Some(&token), None).await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(body["error"]["code"], "CNPJ_NOT_FOUND");
    assert_eq!(calls.load(Ordering::SeqCst), 1);
}

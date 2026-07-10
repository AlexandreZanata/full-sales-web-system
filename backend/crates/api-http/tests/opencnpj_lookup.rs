//! Contract tests: OpenCNPJ adapter (Phase 70C) — mock HTTP, no live keys.

use std::time::Duration;

use api_http::cnpj_lookup::opencnpj::OpenCnpjLookup;
use api_http::cnpj_lookup::{CnpjLookupError, CnpjLookupProvider};
use reqwest::Client;
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn test_client() -> Client {
    Client::builder()
        .timeout(Duration::from_secs(2))
        .build()
        .expect("client")
}

fn sample_cnpj_json() -> String {
    r#"{
        "cnpj": "00000000000191",
        "razao_social": "BANCO DO BRASIL SA",
        "nome_fantasia": "BB",
        "uf": "DF",
        "municipio": "BRASILIA",
        "socios": [],
        "endereco": {
            "logradouro": "SAUN QUADRA 5 BLOCO B TORRE I",
            "numero": "SN",
            "bairro": "ASA NORTE",
            "municipio": "BRASILIA",
            "uf": "DF",
            "cep": "70040-912"
        }
    }"#
    .into()
}

async fn lookup_on_mock(
    server: &MockServer,
    status: u16,
    body: &str,
) -> Result<api_http::cnpj_lookup::CnpjLookupResult, CnpjLookupError> {
    Mock::given(method("GET"))
        .and(path("/api/v1/cnpj/00000000000191"))
        .and(header("X-API-Key", "test-key"))
        .respond_with(ResponseTemplate::new(status).set_body_string(body))
        .mount(server)
        .await;

    let provider = OpenCnpjLookup::new(server.uri(), "test-key".into(), test_client());
    provider.lookup("00000000000191").await
}

#[tokio::test]
async fn contract_opencnpj_when_200_then_normalized_result() {
    let server = MockServer::start().await;
    let result = lookup_on_mock(&server, 200, &sample_cnpj_json()).await;
    let ok = result.expect("lookup ok");
    assert_eq!(ok.cnpj, "00000000000191");
    assert_eq!(ok.legal_name, "BANCO DO BRASIL SA");
    assert_eq!(ok.trade_name, "BB");
    assert_eq!(ok.address.street, "SAUN QUADRA 5 BLOCO B TORRE I");
    assert_eq!(ok.address.postal_code, "70040912");
    assert_eq!(ok.provider, "opencnpj");
    assert!(ok.upstream_snapshot.is_some());
}

#[tokio::test]
async fn contract_opencnpj_when_cnpj_not_found_then_not_found() {
    let server = MockServer::start().await;
    let err = lookup_on_mock(
        &server,
        404,
        r#"{"error":"cnpj_not_found","message":"cnpj not found","code":404}"#,
    )
    .await
    .expect_err("expected not found");
    assert!(matches!(err, CnpjLookupError::NotFound));
}

#[tokio::test]
async fn contract_opencnpj_when_invalid_api_key_then_unavailable() {
    let server = MockServer::start().await;
    let err = lookup_on_mock(
        &server,
        401,
        r#"{"error":"invalid_api_key","message":"invalid api key","code":401}"#,
    )
    .await
    .expect_err("expected unavailable");
    assert!(matches!(err, CnpjLookupError::Unavailable));
}

#[tokio::test]
async fn contract_opencnpj_when_rate_limited_then_unavailable() {
    let server = MockServer::start().await;
    let err = lookup_on_mock(
        &server,
        429,
        r#"{"error":"rate_limit_exceeded","message":"rate limit exceeded","code":429}"#,
    )
    .await
    .expect_err("expected unavailable");
    assert!(matches!(err, CnpjLookupError::Unavailable));
}

#[tokio::test]
async fn contract_opencnpj_when_malformed_json_then_unavailable() {
    let server = MockServer::start().await;
    let err = lookup_on_mock(&server, 200, "not-json")
        .await
        .expect_err("bad json");
    assert!(matches!(err, CnpjLookupError::Unavailable));
}

#[tokio::test]
async fn contract_opencnpj_when_504_then_retries_once() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v1/cnpj/00000000000191"))
        .respond_with(ResponseTemplate::new(504))
        .up_to_n_times(1)
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/api/v1/cnpj/00000000000191"))
        .respond_with(ResponseTemplate::new(200).set_body_string(sample_cnpj_json()))
        .mount(&server)
        .await;

    let provider = OpenCnpjLookup::new(server.uri(), "test-key".into(), test_client());
    let result = provider.lookup("00000000000191").await;
    assert!(result.is_ok());
}

#[test]
fn contract_env_when_opencnpj_selected_then_builds_provider() {
    let result = OpenCnpjLookup::from_config(
        "https://api.comerc.app.br".into(),
        "ocnpj_test_key".into(),
    );
    assert!(result.is_ok());
}

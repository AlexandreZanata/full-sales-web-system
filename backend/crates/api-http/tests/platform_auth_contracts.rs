//! Phase 17J — Platform auth (T-17-105..106, T-17-164..165).
#[path = "support/mod.rs"]
mod support;

use http::StatusCode;
use serde_json::json;

use support::{
    PLATFORM_ADMIN_EMAIL, current_mfa_code, platform_login_step, request, seed_platform_admin,
    setup,
};

async fn mfa_tokens(env: &support::TestEnv) -> (String, String) {
    seed_platform_admin(env).await;
    let login = platform_login_step(env, PLATFORM_ADMIN_EMAIL, "secret123").await;
    let (st, body) = request(
        env,
        "POST",
        "/v1/platform/auth/mfa/verify",
        None,
        Some(
            json!({
                "mfaToken": login["mfaToken"],
                "code": current_mfa_code()
            })
            .to_string(),
        ),
    )
    .await;
    assert_eq!(st, StatusCode::OK);
    (
        body["accessToken"].as_str().expect("access").to_owned(),
        body["refreshToken"].as_str().expect("refresh").to_owned(),
    )
}

// T-17-105 / T-17-106 / T-17-164 / T-17-165
#[tokio::test]
async fn given_platform_admin_when_login_refresh_logout_then_ok() {
    let env = setup().await;
    let (access, refresh) = mfa_tokens(&env).await;

    let (ref_st, refreshed) = request(
        &env,
        "POST",
        "/v1/platform/auth/refresh",
        None,
        Some(json!({ "refreshToken": refresh }).to_string()),
    )
    .await;
    assert_eq!(ref_st, StatusCode::OK, "{refreshed}");
    assert!(refreshed["accessToken"].is_string());
    let new_refresh = refreshed["refreshToken"].as_str().unwrap_or(&refresh);

    let (out_st, _) = request(
        &env,
        "POST",
        "/v1/platform/auth/logout",
        Some(&access),
        Some(json!({ "refreshToken": new_refresh }).to_string()),
    )
    .await;
    assert!(
        out_st == StatusCode::NO_CONTENT || out_st == StatusCode::OK,
        "{out_st}"
    );
}

// T-17-105 errors
#[tokio::test]
async fn given_bad_password_when_platform_login_then_401() {
    let env = setup().await;
    seed_platform_admin(&env).await;
    let (st, body) = request(
        &env,
        "POST",
        "/v1/platform/auth/login",
        None,
        Some(json!({ "email": PLATFORM_ADMIN_EMAIL, "password": "wrong" }).to_string()),
    )
    .await;
    assert_eq!(st, StatusCode::UNAUTHORIZED);
    assert_eq!(body["error"]["code"], "INVALID_CREDENTIALS");
}

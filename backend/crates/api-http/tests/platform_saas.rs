//! Phase 13 — aggregated Platform SaaS contract suite.
//!
//! CI runs with mock Asaas/DNS (testcontainers). Optional live sandbox:
//! `ASAAS_SANDBOX=1 cargo test -p api-http platform_saas_sandbox -- --ignored --test-threads=1`

#[path = "support/mod.rs"]
mod support;

#[path = "platform_saas/helpers.rs"]
mod helpers;

#[path = "platform_saas/lifecycle.rs"]
mod lifecycle;

#[path = "platform_saas/isolation.rs"]
mod isolation;

#[path = "platform_saas/isolation_orders.rs"]
mod isolation_orders;

#[path = "platform_saas/isolation_seed.rs"]
mod isolation_seed;

#[path = "platform_saas/impersonation_audit.rs"]
mod impersonation_audit;

#[path = "platform_saas/webhook_fraud_domain.rs"]
mod webhook_fraud_domain;

#[path = "platform_saas/sandbox.rs"]
mod sandbox;

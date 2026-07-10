use async_trait::async_trait;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DnsError {
    #[error("dns lookup failed")]
    LookupFailed,
}

#[async_trait]
pub trait DnsTxtResolver: Send + Sync {
    async fn lookup_txt(&self, name: &str) -> Result<Vec<String>, DnsError>;
}

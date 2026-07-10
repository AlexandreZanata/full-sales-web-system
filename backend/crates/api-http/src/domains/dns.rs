use std::collections::HashMap;
use std::sync::Mutex;

use application::domains::{DnsError, DnsTxtResolver};
use async_trait::async_trait;

#[derive(Default)]
pub struct MockDnsTxtResolver {
    records: Mutex<HashMap<String, Vec<String>>>,
}

impl MockDnsTxtResolver {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_txt(&self, name: &str, values: Vec<String>) {
        self.records
            .lock()
            .expect("mock dns lock")
            .insert(name.to_lowercase(), values);
    }
}

#[async_trait]
impl DnsTxtResolver for MockDnsTxtResolver {
    async fn lookup_txt(&self, name: &str) -> Result<Vec<String>, DnsError> {
        let guard = self.records.lock().map_err(|_| DnsError::LookupFailed)?;
        Ok(guard.get(&name.to_lowercase()).cloned().unwrap_or_default())
    }
}

pub struct EmptyDnsTxtResolver;

#[async_trait]
impl DnsTxtResolver for EmptyDnsTxtResolver {
    async fn lookup_txt(&self, _name: &str) -> Result<Vec<String>, DnsError> {
        Ok(Vec::new())
    }
}

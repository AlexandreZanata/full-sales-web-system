use std::collections::HashMap;
use std::sync::Mutex;

use domain_shared::TenantId;

/// Stored idempotency replay payload for POST /v1/sales.
#[derive(Debug, Clone)]
pub struct IdempotencyRecord {
    pub status_code: u16,
    pub body: Vec<u8>,
    pub location: Option<String>,
}

pub trait IdempotencyStore: Send + Sync {
    fn get(&self, tenant_id: TenantId, key: &str) -> Option<IdempotencyRecord>;

    fn put(&self, tenant_id: TenantId, key: &str, record: IdempotencyRecord);
}

/// In-memory idempotency store for dev/tests.
#[derive(Default)]
pub struct InMemoryIdempotencyStore {
    records: Mutex<HashMap<(uuid::Uuid, String), IdempotencyRecord>>,
}

impl InMemoryIdempotencyStore {
    pub fn new() -> Self {
        Self::default()
    }
}

impl IdempotencyStore for InMemoryIdempotencyStore {
    fn get(&self, tenant_id: TenantId, key: &str) -> Option<IdempotencyRecord> {
        self.records
            .lock()
            .ok()?
            .get(&(tenant_id.as_uuid(), key.to_owned()))
            .cloned()
    }

    fn put(&self, tenant_id: TenantId, key: &str, record: IdempotencyRecord) {
        if let Ok(mut guard) = self.records.lock() {
            guard.insert((tenant_id.as_uuid(), key.to_owned()), record);
        }
    }
}

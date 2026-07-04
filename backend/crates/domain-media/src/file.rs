use domain_identity::UserId;
use domain_shared::TenantId;
use uuid::Uuid;

use crate::file_entity_type::FileEntityType;
use crate::file_id::FileId;
use crate::upload::validate_upload;

pub struct FileCreateInput {
    pub id: FileId,
    pub tenant_id: TenantId,
    pub entity_type: FileEntityType,
    pub entity_id: Uuid,
    pub bucket: String,
    pub object_key: String,
    pub mime_type: String,
    pub size_bytes: u64,
    pub sha256: String,
    pub uploaded_by_user_id: UserId,
    pub bytes: Vec<u8>,
}

/// Metadata record for an uploaded file — immutable after insert.
#[derive(Debug, Clone)]
pub struct File {
    id: FileId,
    tenant_id: TenantId,
    entity_type: FileEntityType,
    entity_id: Uuid,
    bucket: String,
    object_key: String,
    mime_type: String,
    size_bytes: u64,
    sha256: String,
    uploaded_by_user_id: UserId,
}

impl File {
    pub fn create(input: FileCreateInput) -> Result<Self, crate::error::MediaError> {
        validate_upload(
            &input.mime_type,
            input.size_bytes,
            &input.bytes,
            &input.sha256,
        )?;
        Ok(Self {
            id: input.id,
            tenant_id: input.tenant_id,
            entity_type: input.entity_type,
            entity_id: input.entity_id,
            bucket: input.bucket,
            object_key: input.object_key,
            mime_type: input.mime_type,
            size_bytes: input.size_bytes,
            sha256: input.sha256.to_ascii_lowercase(),
            uploaded_by_user_id: input.uploaded_by_user_id,
        })
    }

    pub fn id(&self) -> FileId {
        self.id
    }

    pub fn tenant_id(&self) -> TenantId {
        self.tenant_id
    }

    pub fn entity_type(&self) -> FileEntityType {
        self.entity_type
    }

    pub fn entity_id(&self) -> Uuid {
        self.entity_id
    }

    pub fn bucket(&self) -> &str {
        &self.bucket
    }

    pub fn object_key(&self) -> &str {
        &self.object_key
    }

    pub fn mime_type(&self) -> &str {
        &self.mime_type
    }

    pub fn size_bytes(&self) -> u64 {
        self.size_bytes
    }

    pub fn sha256(&self) -> &str {
        &self.sha256
    }

    pub fn uploaded_by_user_id(&self) -> UserId {
        self.uploaded_by_user_id
    }
}

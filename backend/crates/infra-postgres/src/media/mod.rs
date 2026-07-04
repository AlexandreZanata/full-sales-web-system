use domain_shared::TenantId;
use sqlx::PgPool;
use uuid::Uuid;

use crate::PostgresError;
use crate::rls::apply_tenant_context;

pub struct FileInsert {
    pub id: Uuid,
    pub entity_type: String,
    pub entity_id: Uuid,
    pub bucket: String,
    pub object_key: String,
    pub mime_type: String,
    pub size_bytes: i64,
    pub sha256: String,
    pub uploaded_by_user_id: Uuid,
}

pub struct FileRow {
    pub id: Uuid,
    pub entity_type: String,
    pub entity_id: Uuid,
    pub bucket: String,
    pub object_key: String,
    pub mime_type: String,
    pub size_bytes: i64,
    pub sha256: String,
    pub uploaded_by_user_id: Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

pub async fn insert_file(
    pool: &PgPool,
    tenant_id: TenantId,
    file: FileInsert,
) -> Result<(), PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    sqlx::query(
        "INSERT INTO media.files
         (id, tenant_id, entity_type, entity_id, bucket, object_key,
          mime_type, size_bytes, sha256, uploaded_by_user_id)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)",
    )
    .bind(file.id)
    .bind(tenant_id.as_uuid())
    .bind(file.entity_type)
    .bind(file.entity_id)
    .bind(file.bucket)
    .bind(file.object_key)
    .bind(file.mime_type)
    .bind(file.size_bytes)
    .bind(file.sha256)
    .bind(file.uploaded_by_user_id)
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(())
}

pub async fn find_file_by_id(
    pool: &PgPool,
    tenant_id: TenantId,
    id: Uuid,
) -> Result<Option<FileRow>, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    let row = sqlx::query_as::<_, (
        Uuid,
        String,
        Uuid,
        String,
        String,
        String,
        i64,
        String,
        Uuid,
        chrono::DateTime<chrono::Utc>,
    )>(
        "SELECT id, entity_type, entity_id, bucket, object_key, mime_type,
                size_bytes, sha256, uploaded_by_user_id, created_at
         FROM media.files WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(row.map(
        |(
            id,
            entity_type,
            entity_id,
            bucket,
            object_key,
            mime_type,
            size_bytes,
            sha256,
            uploaded_by_user_id,
            created_at,
        )| FileRow {
            id,
            entity_type,
            entity_id,
            bucket,
            object_key,
            mime_type,
            size_bytes,
            sha256,
            uploaded_by_user_id,
            created_at,
        },
    ))
}

pub async fn list_files_by_entity(
    pool: &PgPool,
    tenant_id: TenantId,
    entity_type: &str,
    entity_id: Uuid,
) -> Result<Vec<FileRow>, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    let rows = sqlx::query_as::<_, (
        Uuid,
        String,
        Uuid,
        String,
        String,
        String,
        i64,
        String,
        Uuid,
        chrono::DateTime<chrono::Utc>,
    )>(
        "SELECT id, entity_type, entity_id, bucket, object_key, mime_type,
                size_bytes, sha256, uploaded_by_user_id, created_at
         FROM media.files
         WHERE entity_type = $1 AND entity_id = $2
         ORDER BY created_at",
    )
    .bind(entity_type)
    .bind(entity_id)
    .fetch_all(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(rows
        .into_iter()
        .map(
            |(
                id,
                entity_type,
                entity_id,
                bucket,
                object_key,
                mime_type,
                size_bytes,
                sha256,
                uploaded_by_user_id,
                created_at,
            )| FileRow {
                id,
                entity_type,
                entity_id,
                bucket,
                object_key,
                mime_type,
                size_bytes,
                sha256,
                uploaded_by_user_id,
                created_at,
            },
        )
        .collect())
}

pub async fn list_file_ids(pool: &PgPool, tenant_id: TenantId) -> Result<Vec<Uuid>, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    let ids = sqlx::query_scalar::<_, Uuid>("SELECT id FROM media.files ORDER BY created_at")
        .fetch_all(&mut *tx)
        .await?;
    tx.commit().await?;
    Ok(ids)
}

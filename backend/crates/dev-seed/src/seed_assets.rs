//! Bundled JPEG demo assets → local object storage + media.files rows.

use std::path::PathBuf;

use domain_shared::TenantId;
use infra_postgres::PgPool;
use infra_postgres::inventory::product_images::{self, ProductImageInsert};
use infra_postgres::media::{self, FileInsert, FileStorageUpdate};
use infra_storage::{LocalFsObjectStorage, ObjectStorage};
use uuid::Uuid;

use crate::error::{DevSeedError, DevSeedResult};
use crate::media_bytes::{DEV_MEDIA_BUCKET, minimal_webp_bytes};

pub fn mime_for_asset(relative: &str) -> &'static str {
    if relative.ends_with(".png") {
        "image/png"
    } else if relative.ends_with(".webp") {
        "image/webp"
    } else {
        "image/jpeg"
    }
}

pub fn read_asset(relative: &str) -> DevSeedResult<(Vec<u8>, &'static str)> {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("assets")
        .join(relative);
    let bytes = std::fs::read(&path).map_err(|err| {
        DevSeedError::Aborted(format!("missing seed asset {relative}: {err}"))
    })?;
    Ok((bytes, mime_for_asset(relative)))
}

pub fn read_asset_or_placeholder(relative: &str) -> (Vec<u8>, &'static str) {
    read_asset(relative).unwrap_or_else(|_| (minimal_webp_bytes(), "image/webp"))
}

pub async fn ensure_storage_bytes(
    object_key: &str,
    bytes: &[u8],
    mime_type: &str,
) -> DevSeedResult<()> {
    let Some(storage) = open_dev_storage() else {
        return Ok(());
    };
    storage
        .put_object(DEV_MEDIA_BUCKET, object_key, bytes, mime_type)
        .await
        .map_err(|err| DevSeedError::Aborted(format!("storage put {object_key}: {err}")))?;
    Ok(())
}

pub async fn ensure_media_file(
    app_pool: &PgPool,
    admin_pool: &PgPool,
    tenant: TenantId,
    file_id: Uuid,
    entity_type: &str,
    entity_id: Uuid,
    object_key: &str,
    asset_relative: &str,
    uploaded_by: Uuid,
) -> DevSeedResult<()> {
    let (bytes, mime_type) = read_asset_or_placeholder(asset_relative);
    ensure_storage_bytes(object_key, &bytes, mime_type).await?;

    if let Some(existing) = media::find_file_by_id(app_pool, tenant, file_id).await? {
        if existing.object_key != object_key
            || existing.mime_type != mime_type
            || existing.size_bytes != bytes.len() as i64
        {
            media::update_file_storage(
                admin_pool,
                tenant,
                file_id,
                FileStorageUpdate {
                    object_key: object_key.into(),
                    mime_type: mime_type.into(),
                    size_bytes: bytes.len() as i64,
                    sha256: format!("dev-seed-{object_key}"),
                },
            )
            .await?;
        }
        return Ok(());
    }

    media::insert_file(
        app_pool,
        tenant,
        FileInsert {
            id: file_id,
            entity_type: entity_type.into(),
            entity_id,
            bucket: DEV_MEDIA_BUCKET.into(),
            object_key: object_key.into(),
            mime_type: mime_type.into(),
            size_bytes: bytes.len() as i64,
            sha256: format!("dev-seed-{object_key}"),
            uploaded_by_user_id: uploaded_by,
        },
    )
    .await?;
    Ok(())
}

pub async fn ensure_product_image_link(
    app_pool: &PgPool,
    tenant: TenantId,
    image_row_id: Uuid,
    product_id: Uuid,
    file_id: Uuid,
) -> DevSeedResult<()> {
    if product_images::find_product_image_by_id(app_pool, tenant, image_row_id)
        .await?
        .is_some()
    {
        return Ok(());
    }

    let images = product_images::list_product_images(app_pool, tenant, product_id).await?;
    if images.iter().any(|row| row.file_id == file_id) {
        return Ok(());
    }

    if images.iter().any(|row| row.is_primary) {
        product_images::clear_primary_for_product(app_pool, tenant, product_id).await?;
    }

    product_images::insert_product_image(
        app_pool,
        tenant,
        ProductImageInsert {
            id: image_row_id,
            product_id,
            file_id,
            sort_order: 0,
            is_primary: true,
        },
    )
    .await?;
    Ok(())
}

pub fn open_dev_storage() -> Option<LocalFsObjectStorage> {
    if let Ok(path) = std::env::var("MEDIA_LOCAL_PATH") {
        if let Ok(storage) = LocalFsObjectStorage::new(path) {
            return Some(storage);
        }
    }
    LocalFsObjectStorage::new(".local/object-storage").ok()
}

use std::collections::HashMap;
use std::time::Duration;

use domain_shared::TenantId;
use infra_storage::object_storage::DEFAULT_PRESIGN_TTL_SECS;
use uuid::Uuid;

use crate::error::ApiError;
use crate::media::catalog_image_url;
use crate::state::AppState;

#[derive(Clone)]
pub struct PrimaryImageRef {
    pub file_id: Uuid,
    pub url: Option<String>,
}

pub async fn load_primary_images_by_product_id(
    state: &AppState,
    tenant_id: TenantId,
    product_ids: &[Uuid],
) -> Result<HashMap<Uuid, PrimaryImageRef>, ApiError> {
    if product_ids.is_empty() {
        return Ok(HashMap::new());
    }
    let images = infra_postgres::inventory::product_images::find_primary_images_for_products(
        &state.app_pool,
        tenant_id,
        product_ids,
    )
    .await
    .map_err(|_| ApiError::internal())?;

    let ttl = Duration::from_secs(DEFAULT_PRESIGN_TTL_SECS);
    let mut map = HashMap::with_capacity(images.len());
    for image in images {
        let url = state
            .storage
            .presigned_get(&image.bucket, &image.object_key, ttl)
            .await
            .ok()
            .map(|presigned| catalog_image_url(image.file_id, &presigned.url));
        map.insert(
            image.product_id,
            PrimaryImageRef {
                file_id: image.file_id,
                url,
            },
        );
    }
    Ok(map)
}

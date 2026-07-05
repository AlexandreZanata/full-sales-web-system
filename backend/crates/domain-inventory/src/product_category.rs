use domain_shared::TenantId;
use uuid::Uuid;

use crate::category_slug::{slugify_name, unique_slug};
use crate::error::InventoryError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProductCategory {
    id: Uuid,
    tenant_id: TenantId,
    name: String,
    slug: String,
    description: Option<String>,
    sort_order: i32,
    active: bool,
}

pub struct ProductCategoryCreateInput {
    pub id: Uuid,
    pub tenant_id: TenantId,
    pub name: String,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub sort_order: i32,
    pub active: bool,
    pub existing_slugs: Vec<String>,
}

impl ProductCategory {
    pub fn create(input: ProductCategoryCreateInput) -> Result<Self, InventoryError> {
        let name = input.name.trim();
        if name.is_empty() || name.len() > 200 {
            return Err(InventoryError::InvalidCategoryName);
        }

        let base = input
            .slug
            .as_deref()
            .map(slugify_name)
            .filter(|slug| !slug.is_empty())
            .unwrap_or_else(|| slugify_name(name));

        if base.is_empty() {
            return Err(InventoryError::InvalidCategorySlug);
        }

        let slug = unique_slug(&base, &input.existing_slugs);
        let description = input
            .description
            .map(|value| value.trim().to_owned())
            .filter(|value| !value.is_empty());

        Ok(Self {
            id: input.id,
            tenant_id: input.tenant_id,
            name: name.to_owned(),
            slug,
            description,
            sort_order: input.sort_order.max(0),
            active: input.active,
        })
    }

    pub fn rename(
        &mut self,
        name: &str,
        regenerate_slug: bool,
        existing_slugs: &[String],
    ) -> Result<(), InventoryError> {
        let trimmed = name.trim();
        if trimmed.is_empty() || trimmed.len() > 200 {
            return Err(InventoryError::InvalidCategoryName);
        }
        self.name = trimmed.to_owned();
        if regenerate_slug {
            let base = slugify_name(trimmed);
            if base.is_empty() {
                return Err(InventoryError::InvalidCategorySlug);
            }
            let others: Vec<String> = existing_slugs
                .iter()
                .filter(|slug| **slug != self.slug)
                .cloned()
                .collect();
            self.slug = unique_slug(&base, &others);
        }
        Ok(())
    }

    pub fn deactivate(&mut self) {
        self.active = false;
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn tenant_id(&self) -> TenantId {
        self.tenant_id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn slug(&self) -> &str {
        &self.slug
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn sort_order(&self) -> i32 {
        self.sort_order
    }

    pub fn is_active(&self) -> bool {
        self.active
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use domain_shared::TenantId;

    #[test]
    fn given_duplicate_slug_when_create_then_suffix() {
        let tenant = TenantId::generate();
        let category = ProductCategory::create(ProductCategoryCreateInput {
            id: Uuid::now_v7(),
            tenant_id: tenant,
            name: "Bebidas".into(),
            slug: None,
            description: None,
            sort_order: 0,
            active: true,
            existing_slugs: vec!["bebidas".into()],
        })
        .expect("category");
        assert_eq!(category.slug(), "bebidas-2");
    }

    #[test]
    fn given_active_category_when_deactivate_then_inactive() {
        let tenant = TenantId::generate();
        let mut category = ProductCategory::create(ProductCategoryCreateInput {
            id: Uuid::now_v7(),
            tenant_id: tenant,
            name: "Snacks".into(),
            slug: None,
            description: None,
            sort_order: 1,
            active: true,
            existing_slugs: vec![],
        })
        .expect("category");
        category.deactivate();
        assert!(!category.is_active());
    }
}

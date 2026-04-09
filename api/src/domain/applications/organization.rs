use chrono::Utc;
use uuid::Uuid;

use crate::domain::{
    entities::Organization,
    ports::OrganizationPort,
};

pub struct OrganizationApplication<O: OrganizationPort> {
    organization_service: O,
}

impl<O: OrganizationPort> OrganizationApplication<O> {
    pub fn new(organization_service: O) -> Self {
        Self {
            organization_service,
        }
    }

    pub async fn find_all(&self) -> anyhow::Result<Vec<Organization>> {
        self.organization_service.select_all().await
    }

    pub async fn find_by_id(&self, id: Uuid) -> anyhow::Result<Option<Organization>> {
        self.organization_service.select_by_id(id).await
    }

    pub async fn create(
        &self,
        name: String,
        slug: String,
    ) -> anyhow::Result<Organization> {
        let organization = Organization::new(name, slug);

        self.organization_service.insert(organization).await
    }

    pub async fn update(
        &self,
        id: Uuid,
        name: Option<String>,
        slug: Option<String>,
    ) -> anyhow::Result<Option<Organization>> {
        let mut organization = match self.organization_service.select_by_id(id).await? {
            Some(organization) => organization,
            None => return Ok(None),
        };

        let mut has_changed = false;

        if let Some(name) = name {
            organization.name = name;
            has_changed = true;
        }

        if let Some(slug) = slug {
            organization.slug = slug;
            has_changed = true;
        }

        if has_changed {
            organization.updated_at = Some(Utc::now());
            organization = self.organization_service.update(organization).await?;
        }

        Ok(Some(organization))
    }

    pub async fn delete(&self, id: Uuid) -> anyhow::Result<()> {
        self.organization_service.delete(id).await
    }
}

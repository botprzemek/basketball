use chrono::Utc;
use serde::Deserialize;
use uuid::Uuid;

use crate::domain::{entities::Organization, ports::OrganizationPort};

pub struct OrganizationApplication<O: OrganizationPort> {
    organization_service: O,
}

#[derive(Deserialize)]
pub struct CreateOrganization {
    pub name: String,
    pub slug: String,
}

#[derive(Deserialize)]
pub struct UpdateOrganization {
    pub name: Option<String>,
    pub slug: Option<String>,
}

impl<O: OrganizationPort> OrganizationApplication<O> {
    pub fn new(organization_service: O) -> Self {
        Self {
            organization_service,
        }
    }

    pub async fn find_all(&self) -> anyhow::Result<Vec<Organization>> {
        self.organization_service.select().await
    }

    pub async fn find_by_id(&self, id: Uuid) -> anyhow::Result<Option<Organization>> {
        self.organization_service.select_by_self(id).await
    }

    pub async fn create(&self, command: CreateOrganization) -> anyhow::Result<Organization> {
        let organization = Organization::new(command.name, command.slug);

        self.organization_service.insert(organization).await
    }

    pub async fn update(
        &self,
        id: Uuid,
        command: UpdateOrganization,
    ) -> anyhow::Result<Option<Organization>> {
        let mut organization = match self.organization_service.select_by_self(id).await? {
            Some(organization) => organization,
            None => return Ok(None),
        };

        let mut has_changed = false;

        if let Some(name) = command.name {
            organization.name = name;
            has_changed = true;
        }

        if let Some(slug) = command.slug {
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

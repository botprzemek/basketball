use uuid::Uuid;

pub struct IdentitySelectionActor {
    pub account_id: Uuid,
}

pub struct AuthenticatedActor {
    pub account_id: Uuid,
    pub identity_id: Uuid,
    pub organization_id: Uuid,
}

pub enum Actor {
    Selection(IdentitySelectionActor),
    Authorized(AuthenticatedActor),
}

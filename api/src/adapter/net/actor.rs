use uuid::Uuid;

pub struct SelectionActor {
    pub account_id: Uuid,
}

pub struct AuthenticationActor {
    pub account_id: Uuid,
    pub organization_id: Uuid,
}

pub enum Actor {
    Selected(SelectionActor),
    Authorized(AuthenticationActor),
}

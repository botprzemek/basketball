diesel::table! {
    auth.accounts(id) {
        id -> Uuid,
        email -> Text,
        password_hash -> Text,
        first_name -> Text,
        last_name -> Text,
        is_active -> Bool,
        created_at -> Timestamptz,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    auth.organizations(id) {
        id -> Uuid,
        name -> Text,
        slug -> Text,
        is_active -> Bool,
        created_at -> Timestamptz,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    auth.identities(account_id, organization_id) {
        account_id -> Uuid,
        organization_id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    auth.roles(id) {
        id -> Uuid,
        organization_id -> Uuid,
        name -> Text,
        description -> Text,
        created_at -> Timestamptz,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    auth.identities_roles(role_id, account_id, organization_id) {
        role_id -> Uuid,
        account_id -> Uuid,
        organization_id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::joinable!(identities -> accounts(account_id));
diesel::joinable!(identities -> organizations(organization_id));

diesel::joinable!(roles -> organizations(organization_id));

diesel::joinable!(identities_roles -> roles(role_id));
diesel::joinable!(identities_roles -> accounts(account_id));
diesel::joinable!(identities_roles -> organizations(organization_id));

diesel::allow_tables_to_appear_in_same_query!(
    accounts,
    organizations,
    identities,
    roles,
    identities_roles,
);

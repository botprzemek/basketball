diesel::table! {
    auth.accounts(id) {
        id -> Uuid,
        email -> Text,
        password_hash -> Text,
        created_at -> Timestamptz,
        updated_at -> Nullable<Timestamptz>,
        deleted_at -> Nullable<Timestamptz>,
        verified_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    auth.organizations(id) {
        id -> Uuid,
        name -> Text,
        slug -> Text,
        created_at -> Timestamptz,
        updated_at -> Nullable<Timestamptz>,
        deleted_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    auth.members(account_id, organization_id) {
        account_id -> Uuid,
        organization_id -> Uuid,
        given_name -> Text,
        family_name -> Text,
        name -> Text,
        email -> Text,
        phone_number -> Text,
        gender -> Integer,
        birthdate -> Date,
        picture -> Text,
        created_at -> Timestamptz,
        updated_at -> Nullable<Timestamptz>,
        verified_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    auth.groups(id) {
        id -> Uuid,
        organization_id -> Uuid,
        name -> Text,
        description -> Text,
        created_at -> Timestamptz,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    auth.members_groups(group_id, account_id, organization_id) {
        group_id -> Uuid,
        account_id -> Uuid,
        organization_id -> Uuid,
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
    auth.members_roles(role_id, account_id, organization_id) {
        role_id -> Uuid,
        account_id -> Uuid,
        organization_id -> Uuid,
    }
}

diesel::table! {
    auth.permissions(id) {
        id -> Uuid,
        organization_id -> Uuid,
        scope -> Text,
        name -> Text,
        description -> Text,
        created_at -> Timestamptz,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    auth.roles_permissions(role_id, permission_id, organization_id) {
        role_id -> Uuid,
        permission_id -> Uuid,
        organization_id -> Uuid,
    }
}

diesel::joinable!(members -> accounts(account_id));
diesel::joinable!(members -> organizations(organization_id));

diesel::joinable!(roles -> organizations(organization_id));
diesel::joinable!(groups -> organizations(organization_id));
diesel::joinable!(permissions -> organizations(organization_id));

diesel::joinable!(members_groups -> groups(group_id));
diesel::joinable!(members_groups -> accounts(account_id));
diesel::joinable!(members_groups -> organizations(organization_id));

diesel::joinable!(members_roles -> roles(role_id));
diesel::joinable!(members_roles -> accounts(account_id));
diesel::joinable!(members_roles -> organizations(organization_id));

diesel::joinable!(roles_permissions -> roles(role_id));
diesel::joinable!(roles_permissions -> permissions(permission_id));
diesel::joinable!(roles_permissions -> organizations(organization_id));

diesel::allow_tables_to_appear_in_same_query!(
    accounts,
    organizations,
    members,
    groups,
    members_groups,
    roles,
    members_roles,
    permissions,
    roles_permissions,
);

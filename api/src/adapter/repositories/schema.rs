diesel::table! {
    basketball.accounts (id) {
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
    basketball.organizations (id) {
        id -> Uuid,
        name -> Text,
        slug -> Text,
        is_active -> Bool,
        created_at -> Timestamptz,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    basketball.identities (id) {
        id -> Uuid,
        organization_id -> Uuid,
        account_id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Nullable<Timestamptz>,
    }
}


diesel::joinable!(identities -> accounts (account_id));
diesel::joinable!(identities -> organizations (organization_id));

diesel::allow_tables_to_appear_in_same_query!(
    accounts,
    organizations,
    identities,
);
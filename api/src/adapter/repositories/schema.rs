
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
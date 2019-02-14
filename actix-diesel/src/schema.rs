table! {
    categories (id) {
        id -> Int8,
        name -> Varchar,
        description -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        icon_url -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    categories,
);

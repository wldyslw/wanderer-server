table! {
    articles (id) {
        id -> Int4,
        slug -> Text,
        title -> Text,
        title_image -> Text,
        description -> Text,
        body -> Text,
        tag_list -> Array<Text>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        favorites_count -> Int4,
        author -> Int4,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Text,
        password_hash -> Text,
    }
}

joinable!(articles -> users (author));

allow_tables_to_appear_in_same_query!(
    articles,
    users,
);

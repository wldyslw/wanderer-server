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
    }
}

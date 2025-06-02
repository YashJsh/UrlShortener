// @generated automatically by Diesel CLI.

diesel::table! {
    urls (id) {
        id -> Int4,
        original_url -> Text,
        #[max_length = 10]
        short_url -> Text,
        created_at -> Timestamp,
        visits -> Int4,
    }
}

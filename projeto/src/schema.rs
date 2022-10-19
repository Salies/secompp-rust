// @generated automatically by Diesel CLI.

diesel::table! {
    albums (id) {
        id -> Integer,
        artist -> Text,
        title -> Text,
        cover_art_url -> Text,
        label -> Text,
        release_date -> Date,
    }
}

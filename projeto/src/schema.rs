// @generated automatically by Diesel CLI.

diesel::table! {
    albums (id) {
        id -> Int4,
        artist -> Varchar,
        title -> Varchar,
        cover_art_url -> Varchar,
        label -> Varchar,
        release_date -> Date,
    }
}

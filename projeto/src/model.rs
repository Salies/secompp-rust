use diesel::{Insertable, Queryable};
use rocket::serde::{Deserialize, Serialize};
use schema::artists;
use chrono::NaiveDateTime;

// Se algum desses fosse Nullable seria Option
/*
Não vamos fazer isso pq, adivinha? teria que tratar!
tratar aqui, tratar no front...
só tô dizendo pra vocês não acharem que ORM no Rust não tem atributo opcional,
é igualzinho em qualquer outra linguagem, só que né, tem as chatices do Rust
*/
#[derive(Queryable, Serialize)]
pub struct Album {
    pub id: u32,
    pub artist: String,
    pub title: String,
    pub cover_art_url: String,
    pub label: String,
    pub release_date: NaiveDateTime
}

#[derive(Insertable, Deserialize)]
#[table_name = "albums"]
pub struct NewAlbum {
    pub artist: String,
    pub title: String,
    pub cover_art_url: String,
    pub label: String,
    pub release_date: NaiveDateTime
}
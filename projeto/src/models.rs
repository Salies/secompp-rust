use rocket::serde::{Deserialize, Serialize};
use chrono::NaiveDate;

use diesel::{Insertable, Queryable};

use crate::schema::albums;

#[derive(Serialize, Queryable, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Album {
    pub id: i32,
    pub artist: String,
    pub title: String,
    pub cover_art_url: String,
    pub label: String,
    pub release_date: NaiveDate
}

#[derive(Deserialize, Insertable, Debug)]
#[serde(crate = "rocket::serde")]
#[table_name = "albums"]
pub struct NewAlbum {
    pub artist: String,
    pub title: String,
    pub cover_art_url: String,
    pub label: String,
    pub release_date: NaiveDate
}
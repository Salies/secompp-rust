// Declarando nossos módulos
mod schema;
mod model;
mod routes;
// Módulos de Crates externas
use crate::diesel::prelude::*;
use rocket::{ response::status::{Created, NoContent, NotFound}, serde::json::Json };
use rocket_sync_db_pools::{database, diesel};
// Usando nossos módulos
use model::{Album, NewAlbum};
use schema::albums;
use routes::*;

#[database("sqlite_data")]
struct DbConn(diesel::SqliteConnection);

#[rocket::launch]
fn rocket() -> _ {
    rocket::build()
        // State
        .attach(DbConn::fairing())
        // Routes
        .mount(
            "/artists",
            rocket::routes![all, all_by_artist, get_by_id, delete]
        )
}
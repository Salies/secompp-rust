mod schema;
mod model;

use crate::diesel::prelude::*;
use rocket::{ response::status::{Created, NoContent, NotFound}, serde::json::Json };
use model::{Album, NewAlbum};
use schema::albums;
use rocket_sync_db_pools::{database, diesel};

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
            rocket::routes![list, retrieve, create, update, destroy],
        )
}
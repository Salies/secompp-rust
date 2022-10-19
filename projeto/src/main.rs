use rocket::{
    response::status::{Created, NoContent, NotFound},
    serde::json::Json,
};

use diesel::prelude::*;

use music_api::{
    models::{Album, NewAlbum},
    schema::albums,
    ApiError, PgConnection,
};

#[rocket::launch]
fn rocket() -> _ {
    rocket::build()
        // State
        .attach(PgConnection::fairing())
        // Routes
        .mount(
            "/albums",
            rocket::routes![list, retrieve, create, destroy],
        )
}

#[rocket::get("/")]
async fn list(connection: PgConnection) -> Json<Vec<Album>> {
    connection
        .run(|c| albums::table.load(c))
        .await
        .map(Json)
        .expect("Failed to fetch albums")
}

#[rocket::get("/<id>")]
async fn retrieve(
    connection: PgConnection,
    id: i32,
) -> Result<Json<Album>, NotFound<Json<ApiError>>> {
    connection
        .run(move |c| albums::table.filter(albums::id.eq(id)).first(c))
        .await
        .map(Json)
        .map_err(|e| {
            NotFound(Json(ApiError {
                details: e.to_string(),
            }))
        })
}

#[rocket::post("/", data = "<album>")]
async fn create(
    connection: PgConnection,
    album: Json<NewAlbum>,
) -> Result<Created<Json<Album>>, Json<ApiError>> {
    connection
        .run(move |c| {
            diesel::insert_into(albums::table)
                .values(&album.into_inner())
                .get_result(c)
        })
        .await
        .map(|a| Created::new("/").body(Json(a)))
        .map_err(|e| {
            Json(ApiError {
                details: e.to_string(),
            })
        })
}

#[rocket::delete("/<id>")]
async fn destroy(connection: PgConnection, id: i32) -> Result<NoContent, NotFound<Json<ApiError>>> {
    connection
        .run(move |c| {
            let affected = diesel::delete(albums::table.filter(albums::id.eq(id)))
                .execute(c)
                .expect("Connection is broken");
            match affected {
                1 => Ok(()),
                0 => Err("NotFound"),
                _ => Err("???"),
            }
        })
        .await
        .map(|_| NoContent)
        .map_err(|e| {
            NotFound(Json(ApiError {
                details: e.to_string(),
            }))
        })
}

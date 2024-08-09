#[macro_use]
extern crate rocket;

mod db;
mod models;
mod schema;

use db::Db;
use diesel::{dsl::insert_into, prelude::*};
use models::Image;

use schema::images::dsl::*;

use rand::random;
use rocket::http::{ContentType, Status};
use rocket::response::status;
use rocket::serde::json::Json;

#[get("/images?<id>")]
async fn get_image(db: Db, id: i32) -> Result<(ContentType, Vec<u8>), String> {
    db.run(move |conn| {
        images
            .filter(img_id.eq(id))
            .first::<Image>(conn)
            .ok()
            .and_then(|img: Image| std::fs::read(img.img_path).ok())
            .ok_or("Image not found".to_string())
            .map(|img| (ContentType::PNG, img))
    })
    .await
}

#[post("/postimage?<path>")]
async fn post_image(db: Db, path: String) -> Result<Json<usize>, status::Custom<String>> {
    db.run(move |conn| {
        insert_into(images)
            .values(Image {
                img_id: random::<i32>(), // todo uid gen
                img_path: path,
            })
            .execute(conn)
    })
    .await
    .map(Json)
    .map_err(|err| {
        let error_message = format!("Failed to insert image: {:?}", err);
        status::Custom(Status::InternalServerError, error_message)
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Db::fairing())
        .mount("/", routes![get_image, post_image])
}

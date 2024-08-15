#[macro_use]
extern crate rocket;

mod cors;
mod db;
mod models;
mod schema;

use db::Db;
use diesel::delete;
use schema::images::dsl::*;

use diesel::{dsl::insert_into, prelude::*};
use models::Image;
use rand::random;

use rocket::http::{ContentType, Status};
use rocket::response::status::Custom;
use rocket::serde::json::Json;

#[get("/images?<id>")]
async fn get_image(db: Db, id: u8) -> Result<(ContentType, Vec<u8>), String> {
    db.run(move |conn| {
        images
            .filter(img_id.eq(id as i32))
            .first::<Image>(conn)
            .ok()
            .and_then(|img: Image| std::fs::read(img.img_path).ok())
            .ok_or(String::from("Image not found"))
            .map(|img| (ContentType::PNG, img))
    })
    .await
}

#[post("/delete_image?<name>")]
async fn delete_image(db: Db, name: String) -> Result<Custom<String>, Custom<String>> {
    db.run(move |conn| {
        images
            .filter(img_name.eq(&name))
            .first::<Image>(conn)
            .ok()
            .and_then(|img: Image| std::fs::remove_file(img.img_path).ok())
            .ok_or(Custom(Status::BadRequest, String::from("Image not found")))?;
        delete(images.filter(img_name.eq(name)))
            .execute(conn)
            .map_err(|err| Custom(Status::BadRequest, err.to_string()))
    })
    .await?;

    Ok(Custom(Status::Accepted, String::from("Image removed.")))
}

#[get("/all_images")]
async fn get_all_portfolio_images(db: Db) -> Result<Json<Vec<Image>>, Custom<String>> {
    db.run(|conn| images.load::<Image>(conn))
        .await
        .map(Json)
        .map_err(|err| {
            Custom(
                Status::InternalServerError,
                format!("Failed to load images: {:?}", err),
            )
        })
}

#[post("/post_image?<path>&<name>")]
async fn post_image(db: Db, path: String, name: String) -> Result<Custom<String>, Custom<String>> {
    db.run(move |conn| {
        let generated_img_id = random::<u8>() as i32;
        let new_path: String = format!("/home/postgres/images/{}.jpeg", &generated_img_id);
        std::fs::copy(&path, &new_path).map_err(|e| {
            diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::SerializationFailure,
                Box::new(e.to_string()),
            )
        })?;

        insert_into(images)
            .values(Image {
                img_id: generated_img_id, // todo uid gen
                img_path: new_path,
                img_name: name,
            })
            .execute(conn)
    })
    .await
    .map(Json)
    .map_err(|err| {
        Custom(
            Status::InternalServerError,
            format!("Failed to insert image: {:?}", err),
        )
    })?;

    Ok(Custom(
        Status::Accepted,
        String::from("Image added to the db."),
    ))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Db::fairing())
        .attach(cors::CORS)
        .mount(
            "/",
            routes![
                get_image,
                post_image,
                get_all_portfolio_images,
                delete_image
            ],
        )
}

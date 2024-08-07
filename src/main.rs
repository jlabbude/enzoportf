#[macro_use]
extern crate rocket;

mod db;
mod models;
mod schema;

use db::Db;
use diesel::prelude::*;
use models::Image;
use rocket::http::ContentType;
use rocket::response::content;

#[get("/images/<id>")]
async fn get_image(db: Db, id: i32) -> Option<content<Vec<u8>>> {
    db.run(move |conn| {
        use schema::images::dsl::*;
        images
            .filter(id.eq(id))
            .first::<Image>(conn)
            .ok()
            .map(|img| content::RawHtml("<img{0}>".replace("{0}", std::fs::read(img.img_path)?)))
    })
    .await
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Db::fairing())
        .mount("/", routes![get_image])
}

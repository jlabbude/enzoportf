#[macro_use]
extern crate rocket;

mod db;
mod models;
mod schema;

use db::Db;
use diesel::prelude::*;
use models::Image;

#[get("/images/<id>")]
async fn get_image(db: Db, id: i32) -> Option<Vec<u8>> {
    db.run(move |conn| {
        use schema::images::dsl::*;
        images
            .filter(img_id.eq(id))
            .first::<Image>(conn)
            .ok()
            .map(|img: Image| std::fs::read(img.img_path).unwrap())
    })
    .await
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Db::fairing())
        .mount("/", routes![get_image])
}

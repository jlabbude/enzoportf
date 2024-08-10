#[derive(diesel::Queryable, diesel::Insertable)]
#[diesel(table_name = crate::schema::images)]
#[derive(rocket::serde::Serialize, rocket::serde::Deserialize)]
pub struct Image {
    pub img_id: i32,
    pub img_path: String,
}

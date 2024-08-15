#[derive(
    diesel::Queryable,
    diesel::Insertable,
    rocket::serde::Serialize,
    rocket::serde::Deserialize,
    diesel::Selectable,
)]
#[diesel(table_name = crate::schema::images)]
pub struct Image {
    pub img_id: i32,
    pub img_path: String,
    pub img_name: String,
}

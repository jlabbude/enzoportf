#[derive(diesel::Queryable, diesel::Insertable)]
#[diesel(table_name = crate::schema::images)]
pub struct Image {
    pub img_id: i32,
    pub img_path: String,
}

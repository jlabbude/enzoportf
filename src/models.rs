#[derive(diesel::Queryable)]
pub struct Image {
    pub img_id: i32,
    pub img_path: String,
}

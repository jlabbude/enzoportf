#[derive(diesel::Queryable)]
pub struct Image {
    pub id: i32,
    pub img_path: std::path::Path,
}

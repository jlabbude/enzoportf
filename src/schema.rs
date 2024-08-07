// @generated automatically by Diesel CLI.

diesel::table! {
    images (id) {
        id -> Int4,
        #[max_length = 255]
        img_path -> Varchar,
    }
}

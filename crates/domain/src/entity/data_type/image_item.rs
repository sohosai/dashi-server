use axum::body::Bytes;

#[derive(Clone, Debug)]
pub struct ImageItemData {
    pub id: i32,
    pub image: Bytes,
}

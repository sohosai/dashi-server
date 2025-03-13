use super::webp::convert_to_webp;
use crate::connection;
use domain::{
    repository::connection::ConnectionRepository, value_object::error::object_strage::r2::R2Error,
};

pub async fn upload(id: i32, binary: &[u8]) -> Result<(), R2Error> {
    let object = connection::CollectConnection::connect_object_strage().await?;
    let webp = convert_to_webp(binary, 75.0)?;
    object
        .upload_binary(&format!("{}.webp", id), "image/webp", &webp, None)
        .await?;
    Ok(())
}

pub async fn delete(id: i32) -> Result<(), R2Error> {
    let object = connection::CollectConnection::connect_object_strage().await?;
    object.delete(&format!("{}.webp", id)).await?;
    Ok(())
}

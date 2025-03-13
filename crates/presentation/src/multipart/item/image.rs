use application::usecase::item::image::ImageItemDataJson;
use axum::extract::Multipart;
use domain::value_object::error::item::image::ImageItemError;

pub async fn multipart_image(
    mut multipart: Multipart,
) -> Result<ImageItemDataJson, ImageItemError> {
    let mut image_item_data_json = ImageItemDataJson::default();
    // multipartを一つずつ取り出す
    while let Some(field) = multipart.next_field().await? {
        // fieldの名前を取得してそれぞれ処理する
        match field.name() {
            Some("image") => {
                let binary = field.bytes().await?;
                image_item_data_json.image = binary;
            }
            Some(param_name) => {
                return Err(ImageItemError::UnknownParameterError(
                    param_name.to_string(),
                ));
            }
            None => {
                return Err(ImageItemError::ParameterNotFoundError);
            }
        }
    }
    Ok(image_item_data_json)
}

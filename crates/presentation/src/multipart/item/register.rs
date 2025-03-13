use axum::extract::Multipart;
use domain::{
    entity::data_type::register_item::RegisterItemData,
    value_object::error::item::register::RegisterItemError,
};

pub async fn multipart_register(
    mut multipart: Multipart,
) -> Result<RegisterItemData, RegisterItemError> {
    let mut register_item_data = RegisterItemData::default();
    // multipartを一つずつ取り出す
    while let Some(field) = multipart.next_field().await? {
        // fieldの名前を取得してそれぞれ処理する
        match field.name() {
            Some("parent_visible_id") => {
                let parent_visible_id = field.text().await?;
                println!("parent_visible_id: {}", parent_visible_id);
                register_item_data.parent_visible_id = parent_visible_id;
            }
            Some("visible_id") => {
                let visible_id = field.text().await?;
                println!("visible_id: {}", visible_id);
                register_item_data.visible_id = visible_id;
            }
            Some("name") => {
                let name = field.text().await?;
                println!("name: {}", name);
                register_item_data.name = name;
            }
            Some("product_number") => {
                let product_number = field.text().await?;
                println!("product_number: {}", product_number);
                register_item_data.product_number = product_number;
            }
            Some("description") => {
                let description = field.text().await?;
                println!("description: {}", description);
                register_item_data.description = description;
            }
            Some("purchase_year") => {
                let purchase_year = field.text().await?;
                println!("purchase_year: {}", purchase_year);
                register_item_data.purchase_year = if !purchase_year.is_empty() {
                    Some(
                        purchase_year
                            .parse::<i32>()
                            .map_err(|_e| RegisterItemError::ParseIntError)?,
                    )
                } else {
                    None
                }
            }
            Some("purchase_price") => {
                let purchase_price = field.text().await?;
                println!("purchase_price: {}", purchase_price);
                register_item_data.purchase_price = if !purchase_price.is_empty() {
                    Some(
                        purchase_price
                            .parse::<i32>()
                            .map_err(|_e| RegisterItemError::ParseIntError)?,
                    )
                } else {
                    None
                }
            }
            Some("durability") => {
                let durability = field.text().await?;
                println!("durability: {}", durability);
                register_item_data.durability = if !durability.is_empty() {
                    Some(
                        durability
                            .parse::<i32>()
                            .map_err(|_e| RegisterItemError::ParseIntError)?,
                    )
                } else {
                    None
                }
            }
            Some("is_depreciation") => {
                let is_depreciation = field.text().await?;
                println!("is_depreciation: {}", is_depreciation);
                register_item_data.is_depreciation = if is_depreciation == "true" {
                    true
                } else if is_depreciation == "false" {
                    false
                } else {
                    return Err(RegisterItemError::ParseBooleanError);
                };
            }
            Some("connector") => {
                let raw_connector = field.text().await?;
                println!("connector: {}", raw_connector);
                register_item_data.connector = raw_connector
                    .split("^")
                    .skip(1)
                    .map(|s| s.to_string())
                    .collect();
                println!("{:#?}", register_item_data.connector);
            }
            Some("color") => {
                let color = field.text().await?;
                println!("color: {}", color);
                register_item_data.color = color;
            }
            Some("image") => {
                let binary = field.bytes().await?;
                register_item_data.image = binary;
            }
            Some(param_name) => {
                return Err(RegisterItemError::UnknownParameterError(
                    param_name.to_string(),
                ));
            }
            None => {
                return Err(RegisterItemError::ParameterNotFoundError);
            }
        }
    }
    Ok(register_item_data)
}

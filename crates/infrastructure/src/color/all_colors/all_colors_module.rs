use domain::{
    entity::data_type::color::ColorData, value_object::error::color::all_colors::AllColorsError,
};
use entity::color::Entity as Color;
use sea_orm::{DatabaseConnection, EntityTrait};

pub(super) async fn all_colors(rdb: DatabaseConnection) -> Result<Vec<ColorData>, AllColorsError> {
    let all_colors_data: Vec<ColorData> = Color::find()
        .all(&rdb)
        .await?
        .into_iter()
        .map(|item| ColorData {
            id: item.id,
            name: item.name,
            hex_color_code: item.hex_color_code,
            status: item.status,
        })
        .collect();
    Ok(all_colors_data)
}

use domain::{
    entity::data_type::item_csv::ItemCsvData, value_object::error::csv::item::ItemCsvError,
};
use entity::item::Entity as Item;
use sea_orm::{DatabaseConnection, EntityTrait};

pub(super) async fn item(rdb: DatabaseConnection) -> Result<Vec<ItemCsvData>, ItemCsvError> {
    ////* operation *////
    // get all items
    let item_models = match Item::find().all(&rdb).await {
        Ok(item_models) => item_models,
        Err(e) => return Err(ItemCsvError::RDBError(e)),
    };
    // convert to item csv data
    let item_csv_data = item_models
        .into_iter()
        .map(|item_model| ItemCsvData {
            name: item_model.name,
            product_number: item_model.product_number,
            description: item_model.description,
        })
        .collect();

    Ok(item_csv_data)
}

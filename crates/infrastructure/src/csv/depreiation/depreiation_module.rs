use domain::{
    entity::data_type::depreiation_csv::DepreiationCsvData,
    value_object::error::csv::depreiation::DepreiationCsvError,
};
use entity::item::{self, Entity as Item};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

pub(super) async fn depreiation(
    rdb: DatabaseConnection,
) -> Result<Vec<DepreiationCsvData>, DepreiationCsvError> {
    ////* operation *////
    // get all items that are depreiation
    let depreiation_item_models = match Item::find()
        .filter(item::Column::IsDepreciation.eq(true))
        .all(&rdb)
        .await
    {
        Ok(item_models) => item_models,
        Err(e) => return Err(DepreiationCsvError::RDBError(e)),
    };
    // convert to depreiation csv data
    let depreiation_csv_data = depreiation_item_models
        .into_iter()
        .map(|item_model| DepreiationCsvData {
            name: item_model.name,
            product_number: item_model.product_number,
            purchase_year: item_model.purchase_year,
            purchase_price: item_model.purchase_price,
            durability: item_model.durability,
        })
        .collect();

    Ok(depreiation_csv_data)
}

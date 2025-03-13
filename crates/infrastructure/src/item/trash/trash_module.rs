use domain::{
    entity::data_type::trash_item::TrashItemData, value_object::error::item::trash::TrashItemError,
};
use entity::trash::Entity as Trash;
use sea_orm::{DatabaseConnection, EntityTrait};

pub(super) async fn trash(rdb: DatabaseConnection) -> Result<Vec<TrashItemData>, TrashItemError> {
    ////* operation *////
    let trash_items = match Trash::find().all(&rdb).await {
        Ok(item_models) => item_models,
        Err(e) => return Err(TrashItemError::RDBError(e)),
    };
    //* convert Trash to TrashItemData *//
    let mut trash_item_datas: Vec<TrashItemData> = Vec::new();
    for trash_item in trash_items {
        let connector: Vec<String> =
            serde_json::from_value(trash_item.connector).map_err(TrashItemError::SerdeJsonError)?;
        let trash_item_data: TrashItemData = TrashItemData {
            id: trash_item.id,
            item_id: trash_item.item_id,
            visible_id: trash_item.visible_id,
            name: trash_item.name,
            product_number: trash_item.product_number,
            description: trash_item.description,
            purchase_year: trash_item.purchase_year,
            purchase_price: trash_item.purchase_price,
            durability: trash_item.durability,
            is_depreciation: trash_item.is_depreciation,
            connector,
            is_rent: trash_item.is_rent,
            color: trash_item.color,
            created_at: trash_item.created_at.to_string(),
            updated_at: trash_item.updated_at.to_string(),
        };
        trash_item_datas.push(trash_item_data);
    }
    Ok(trash_item_datas)
}

use domain::{
    entity::data_type::generate::GenerateData,
    value_object::error::{critical_incident, generate::GenerateError},
};
use entity::{
    active_enum::Record,
    label::{self, Entity as Label},
};
use radix_fmt::radix_36;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter,
    Set,
};

pub(super) async fn generate(
    rdb: DatabaseConnection,
    quantity: u32,
    qr_or_barcode: Record,
) -> Result<GenerateData, GenerateError> {
    //* validation *//
    let max_label_models = Label::find()
        .filter(label::Column::IsMax.eq(true))
        .all(&rdb)
        .await?;
    // validation of IsMax
    if max_label_models.len() != 1 {
        return Err(GenerateError::IsMaxBreakError(format!(
            "{}",
            max_label_models.len()
        )));
    }
    let max_label_model: label::Model = max_label_models[0].to_owned();
    // validation of Underflow
    //*! このエラーは不要になる
    if quantity == 0 {
        return Err(GenerateError::UnderflowError(format!("{}", quantity)));
    }
    // validation of Overflow
    let visible_id_10bit = u32::from_str_radix(&max_label_model.visible_id, 36)?;
    // maxmuim isuue limit is 36^4 - 1
    let max = 36u32.pow(4) - 1;
    if visible_id_10bit + quantity > max {
        return Err(GenerateError::OverflowError(format!(
            "{}",
            max - visible_id_10bit
        )));
    }
    //* operation *//
    // update IsMax of current max label
    let mut max_label_model = max_label_model.into_active_model();
    max_label_model.is_max = Set(false);
    let max_label_model = match max_label_model.update(&rdb).await {
        Ok(result) => {
            tracing::info!("IsMax of current max label is updated.");
            tracing::debug!("{:#?}", result);
            result
        }
        Err(e) => {
            tracing::error!("Failed to update IsMax of current max label.");
            return Err(GenerateError::RDBError(e));
        }
    };

    // generate new labels
    let mut new_label_models: Vec<label::ActiveModel> = Vec::new();
    let mut new_label_visible_ids: Vec<String> = Vec::new();
    for i in 1..=quantity {
        let visible_id = format!(
            "{:0>4}",
            radix_36(visible_id_10bit + i).to_string().to_uppercase()
        );

        new_label_visible_ids.push(visible_id.to_owned());
        if i == quantity {
            // IsMax: true (last)
            let insert_label_model = label::ActiveModel {
                visible_id: Set(visible_id.to_owned()),
                is_max: Set(true),
                record: Set(qr_or_barcode.to_owned()),
            };
            new_label_models.push(insert_label_model);
        } else {
            // IsMax: false (not last)
            let insert_label_model = label::ActiveModel {
                visible_id: Set(visible_id.to_owned()),
                is_max: Set(false),
                record: Set(qr_or_barcode.to_owned()),
            };
            new_label_models.push(insert_label_model);
        }
    }
    match Label::insert_many(new_label_models).exec(&rdb).await {
        Ok(result) => {
            tracing::info!("New labels are generated.");
            tracing::debug!("{:#?}", result);
        }
        Err(e) => {
            tracing::error!("Failed to generate new labels.");
            // try rollback
            let mut max_label_model = max_label_model.into_active_model();
            max_label_model.is_max = Set(true);
            let max_label_model = match max_label_model.update(&rdb).await {
                Ok(result) => result,
                Err(e) => {
                    critical_incident::rollback_error().await;
                    tracing::error!(
                        "Failed to rollback IsMax of current max label in Label Table."
                    );
                    return Err(GenerateError::RDBError(e));
                }
            };
            tracing::debug!("Rollbacked IsMax of current max label in Label Table.");
            tracing::debug!("{:#?}", max_label_model);
            return Err(GenerateError::RDBError(e));
        }
    }

    Ok(GenerateData {
        visible_ids: new_label_visible_ids,
    })
}

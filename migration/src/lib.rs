pub use sea_orm_migration::prelude::*;
mod m20220101_000001_color_table;
mod m20220101_000001_connector_table;
mod m20220101_000001_item_table;
mod m20220101_000001_label_table;
mod m20220101_000001_trash_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_label_table::Migration),
            Box::new(m20220101_000001_item_table::Migration),
            Box::new(m20220101_000001_trash_table::Migration),
            Box::new(m20220101_000001_connector_table::Migration),
            Box::new(m20220101_000001_color_table::Migration),
        ]
    }
}

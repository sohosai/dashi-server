use crate::{
    color::{
        all_colors::AllColors, register::RegisterColor, search::SearchColor, update::UpdateColor,
    },
    connector::{
        all_connectors::AllConnectors, register::RegisterConnector, search::SearchConnector,
        status::StatusConnector,
    },
    csv::{depreiation::DepreiationCsv, item::ItemCsv},
    generate::Generate,
    healthcheck::HealthCheck,
    item::{
        delete::DeleteItem, image::ImageItem, individual_item::IndividualItem,
        register::RegisterItem, search::SearchItem, transfer::TransferItem, trash::TrashItem,
        update::UpdateItem,
    },
    rental::{
        all_rental_items::AllRentalItems, rent::RentRental, replace::ReplaceRental,
        update::UpdateRental,
    },
};
use domain::{
    factory::shared_state::SharedStateFactory,
    repository::{
        color::{
            all_colors::AllColorsRepository, register::RegisterColorRepository,
            search::SearchColorRepository, update::UpdateColorRepository,
        },
        connector::{
            all_connectors::AllConnectorsRepository, register::RegisterConnectorRepository,
            search::SearchConnectorRepository, status::StatusConnectorRepository,
        },
        csv::{depreiation::DepreiationCsvRepository, item::ItemCsvRepository},
        generate::GenerateRepository,
        healthcheck::HealthCheckRepository,
        item::{
            delete::DeleteItemRepository, image::ImageItemRepository,
            individual::IndividualItemRepository, register::RegisterItemRepository,
            search::SearchItemRepository, transfer::TransferItemRepository,
            trash::TrashItemRepository, update::UpdateItemRepository,
        },
        rental::{
            all_rental_items::AllRentalItemsRepository, rent::RentRentalRepository,
            replace::ReplaceRentalRepository, update::UpdateRentalRepository,
        },
    },
};

#[derive(Clone)]
pub struct SharedState {
    pub all_rental_items: AllRentalItems,
    pub rent_rental: RentRental,
    pub update_rental: UpdateRental,
    pub replace_rental: ReplaceRental,
    pub update_color: UpdateColor,
    pub search_color: SearchColor,
    pub all_colors: AllColors,
    pub register_color: RegisterColor,
    pub status_connector: StatusConnector,
    pub search_connector: SearchConnector,
    pub all_connectors: AllConnectors,
    pub register_connector: RegisterConnector,
    pub item_csv: ItemCsv,
    pub depreiation_csv: DepreiationCsv,
    pub trash_item: TrashItem,
    pub transfer_item: TransferItem,
    pub individual_item: IndividualItem,
    pub search_item: SearchItem,
    pub update_item: UpdateItem,
    pub image_item: ImageItem,
    pub delete_item: DeleteItem,
    pub register_item: RegisterItem,
    pub generate: Generate,
    pub healthcheck: HealthCheck,
}

impl SharedStateFactory for SharedState {
    async fn new() -> Self {
        SharedState {
            all_rental_items: AllRentalItems::new().await,
            rent_rental: RentRental::new().await,
            update_rental: UpdateRental::new().await,
            replace_rental: ReplaceRental::new().await,
            update_color: UpdateColor::new().await,
            search_color: SearchColor::new().await,
            all_colors: AllColors::new().await,
            register_color: RegisterColor::new().await,
            status_connector: StatusConnector::new().await,
            search_connector: SearchConnector::new().await,
            all_connectors: AllConnectors::new().await,
            register_connector: RegisterConnector::new().await,
            item_csv: ItemCsv::new().await,
            depreiation_csv: DepreiationCsv::new().await,
            trash_item: TrashItem::new().await,
            transfer_item: TransferItem::new().await,
            individual_item: IndividualItem::new().await,
            search_item: SearchItem::new().await,
            update_item: UpdateItem::new().await,
            image_item: ImageItem::new().await,
            delete_item: DeleteItem::new().await,
            register_item: RegisterItem::new().await,
            generate: Generate::new().await,
            healthcheck: HealthCheck::new().await,
        }
    }
}

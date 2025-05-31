use crate::{
    error::api::ApiError, handlers::ping::ping_handler, middlewares::logging::logging_middleware,
    routes,
};
use application::model::shared_state::SharedStateUseCase;
use async_std::sync::{Arc, RwLock};
use axum::{
    extract::DefaultBodyLimit,
    http::{header, HeaderValue, Method},
    middleware,
    routing::get,
    Router,
};
use domain::factory::shared_state::SharedStateFactory;
use infrastructure::shared_state::SharedState;
use tower_http::cors::CorsLayer;
use utoipa::{
    openapi::{
        self,
        security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
    },
    Modify, OpenApi,
};

//axum
pub async fn api() -> Result<(), ApiError> {
    // tracing
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(tracing::Level::DEBUG)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    // Shared Object
    let shared_state = Arc::new(RwLock::new(
        SharedStateUseCase::new(SharedState::new().await)
            .await
            .shared_state_factory,
    ));

    // CORS
    let cors: CorsLayer = CorsLayer::new()
        .allow_headers([header::CONTENT_TYPE])
        .allow_credentials(true)
        .allow_methods([
            Method::POST,
            Method::GET,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
        ])
        .allow_origin([
            "http://localhost:5137".parse::<HeaderValue>().unwrap(),
            "https://dashi.sohosai.com".parse::<HeaderValue>().unwrap(),
        ]);

    // Router
    let app: Router<()> = Router::new()
        .route("/", get(ping_handler))
        .merge(routes::root::root_route())
        .layer(cors)
        .layer(middleware::from_fn(logging_middleware))
        .layer(DefaultBodyLimit::max(1024 * 1024 * 100)) //100MB
        .with_state(Arc::clone(&shared_state));

    // Server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:5000").await?;
    tracing::debug!("listening on http://{}", listener.local_addr()?);
    axum::serve(listener, app).await?;
    Ok(())
}

#[derive(OpenApi)]
#[openapi(
    info(
        title = "dashi-server",
        version = "0.0.1",
        description = "This is a dashi-server API document.",
        contact(
            name = "Myxogastria0808",
            email = "r.rstudio.c@gmail.com",
            url = "https://yukiosada.work",
        ),
        license(
            name = "WTFPL",
            url = "http://www.wtfpl.net"
        ),
    ),
    servers((url = "https://dashi-api.sohosai.com")),
    tags(
        (name = "Item", description = "物品に関係するエンドポイント"),
        (name = "Csv", description = "csv出力に関するエンドポイント"),
        (name = "Rental", description = "レンタルに関係するエンドポイント"),
        (name = "Generate", description = "QRまたはBarcodeを生成するエンドポイント"),
        (name = "Connector", description = "接続端子に関係するエンドポイント"),
        (name = "Color", description = "ケーブルに貼るテープに関係するエンドポイント"),
        (name = "Health Check", description = "Health Checkのエンドポイント"),
        (name = "Ping", description = "pingを送るエンドポイント"),
        (name = "Joke", description = "特殊なステータスコードを返すエンドポイント"),
    ),
    paths(
        crate::handlers::rental::all_rental_items_handler,
        crate::handlers::rental::rent_handler,
        crate::handlers::rental::update_handler,
        crate::handlers::rental::replace_handler,
        crate::handlers::color::update_handler,
        crate::handlers::color::search_handler,
        crate::handlers::color::all_colors_handler,
        crate::handlers::color::register_handler,
        crate::handlers::connector::status_handler,
        crate::handlers::connector::search_handler,
        crate::handlers::connector::all_connectors_handler,
        crate::handlers::connector::register_handler,
        crate::handlers::utils::healthcheck_handler,
        crate::handlers::utils::generate_handler,
        crate::handlers::item::delete_handler,
        crate::handlers::item::register_handler,
        crate::handlers::item::update_handler,
        crate::handlers::item::image_handler,
        crate::handlers::item::search_handler,
        crate::handlers::item::individual_item_handler,
        crate::handlers::item::transfer_handler,
        crate::handlers::item::trash_handler,
        crate::handlers::csv::depreiation_handler,
        crate::handlers::csv::item_handler,
        crate::handlers::joke::unavailable_handler,
        crate::handlers::joke::teapot_handler,
        crate::handlers::ping::ping_handler,
    ),
    components(schemas(
        entity::active_enum::Record,
        entity::active_enum::Status,
        domain::entity::data_type::generate::GenerateData,
        domain::value_object::error::ResponseError,
        domain::entity::data_type::search_item::SearchItemData,
        application::usecase::item::search::SearchItemJson,
        application::usecase::item::individual::IndividualItemDataJson,
        application::usecase::csv::depreiation::DepreiationCsvJson,
        domain::entity::data_type::depreiation_csv::DepreiationCsvData,
        application::usecase::csv::item::ItemCsvJson,
        domain::entity::data_type::item_csv::ItemCsvData,
        domain::entity::data_type::transfer_item::TransferItemData,
        domain::entity::data_type::generate_data_request::GenerateDataRequest,
        application::usecase::item::trash::TrashItemDataJson,
        domain::entity::data_type::register_connector::RegisterConnectorData,
        application::usecase::connector::all_connectors::AllConnectorsJson,
        application::usecase::connector::search::SearchConnectorJson,
        domain::entity::data_type::status_connector::StatusConnectorData,
        domain::entity::data_type::connector::ConnectorData,
        domain::entity::data_type::register_color::RegisterColorData,
        application::usecase::color::all_colors::AllColorsJson,
        application::usecase::color::search::SearchColorJson,
        domain::entity::data_type::update_color::UpdateColorData,
        domain::entity::data_type::color::ColorData,
        domain::entity::data_type::rental::RentalData,
        application::usecase::item::update::UpdateItemDataJson,
        crate::models::register_item_multipart_data::RegisterItemMultipartData,
        crate::models::image_item_multipart::ImageItemMultipartData,
        domain::entity::data_type::trash_item::TrashItemData,
        domain::entity::data_type::rental_item::RentalItemData,
        application::usecase::rental::all_rental_items::RentalItemJson,
    )),
    modifiers(&SecurityAddon),
)]
pub struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut openapi::OpenApi) {
        if let Some(schema) = openapi.components.as_mut() {
            schema.add_security_scheme(
                "jwt_token",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build(),
                ),
            );
        }
    }
}

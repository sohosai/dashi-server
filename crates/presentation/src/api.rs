use application::model::shared_state::SharedStateUseCase;
use async_std::sync::{Arc, RwLock};
use axum::{
    extract::DefaultBodyLimit,
    http::{header, Method},
    routing::get,
    Router,
};
use tower_http::cors::{Any, CorsLayer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

// レイヤードアーキテクチャに違反しているが、Rustの性質上不可能なのでinfrastructure層及びdomain層から直接呼び出す
use crate::{error::api::ApiError, handlers::ping::ping_handler, routes};
use domain::factory::shared_state::SharedStateFactory;
use infrastructure::shared_state::SharedState;

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
        .allow_headers([header::AUTHORIZATION, header::CONTENT_TYPE])
        .expose_headers([header::CONTENT_DISPOSITION])
        .allow_methods([Method::POST, Method::GET, Method::PATCH, Method::DELETE])
        .allow_origin(Any);
    // Router
    let app: Router<()> = Router::new()
        .route("/", get(ping_handler))
        .merge(routes::root::root_route())
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .layer(cors)
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
        description = "Thsi is a dashi-server API document.",
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
    servers((url = "http://0.0.0.0:5000")),
    tags(
        (name = "Item", description = "物品に関係するエンドポイント"),
        (name = "Csv", description = "csv出力に関するエンドポイント"),
        (name = "Generate", description = "QRまたはBarcodeを生成するエンドポイント"),
        (name = "Health Check", description = "Health Checkのエンドポイント"),
        (name = "Ping", description = "pingを送るエンドポイント"),
        (name = "Joke", description = "特殊なステータスコードを返すエンドポイント"),
    ),
    paths(
        crate::handlers::utils::healthcheck_handler,
        crate::handlers::utils::generate_handler,
        crate::handlers::item::delete_handler,
        crate::handlers::item::register_handler,
        crate::handlers::item::update_handler,
        crate::handlers::item::search_handler,
        crate::handlers::item::individual_item_handler,
        crate::handlers::item::transfer_handler,
        crate::handlers::csv::depreiation_handler,
        crate::handlers::csv::item_handler,
        crate::handlers::joke::unavailable_handler,
        crate::handlers::joke::teapot_handler,
        crate::handlers::ping::ping_handler,
    ),
    components(schemas(
        entity::label::Record,
        domain::entity::data_type::generate::GenerateData,
        domain::value_object::error::ResponseError,
        domain::entity::data_type::register_item::RegisterItemData,
        domain::entity::data_type::search_item::SearchItemData,
        application::usecase::item::search::SearchItemJson,
        application::usecase::item::update::UpdateItemDataJson,
        application::usecase::item::individual::IndividualItemDataJson,
        application::usecase::csv::depreiation::DepreiationCsvJson,
        domain::entity::data_type::depreiation_csv::DepreiationCsvData,
        application::usecase::csv::item::ItemCsvJson,
        domain::entity::data_type::item_csv::ItemCsvData,
        domain::entity::data_type::transfer_item::TransferItemData,
        domain::entity::data_type::generate_data_request::GenerateDataRequest,
    ))
)]
struct ApiDoc;

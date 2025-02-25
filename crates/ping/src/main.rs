use domain::repository::connection::ConnectionRepository;
use error::ping::PingError;
use infrastructure::connection;
use neo4rs::query;

mod error;

#[tokio::main]
async fn main() -> Result<(), PingError> {
    // tracing
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(tracing::Level::DEBUG)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("Failed to set subscriber");
    // Connect rdb
    let rdb = match connection::CollectConnection::connect_rdb().await {
        Ok(rdb) => rdb,
        Err(e) => {
            tracing::error!("Failed to connect to PostgreSQL.");
            return Err(PingError::ConnectionError(e));
        }
    };
    // Connect graphdb
    let graphdb = match connection::CollectConnection::connect_graphdb().await {
        Ok(graphdb) => graphdb,
        Err(e) => {
            tracing::error!("Failed to connect to Neo4j.");
            return Err(PingError::ConnectionError(e));
        }
    };
    // Connect meilisearch
    let meilisearch = match connection::CollectConnection::connect_meilisearch().await {
        Ok(meilisearch) => meilisearch,
        Err(e) => {
            tracing::error!("Failed to connect to Meilisearch.");
            return Err(PingError::ConnectionError(e));
        }
    };
    //* ping GraphDB *//
    // get (item:Item {id: 1}) test (if connect to graphdb, it's healthy without {id: 1} node)
    match graphdb
        .execute(query("MATCH (item:Item {id: $id}) RETURN item").param("id", 1))
        .await
    {
        Ok(_) => {
            tracing::info!("GraphDB is healthy.");
        }
        Err(e) => {
            tracing::error!("Failed to ping GraphDB.");
            return Err(PingError::GraphDBError(e));
        }
    };
    //* ping MeiliSearch *//
    match meilisearch.health().await {
        Ok(_) => {
            tracing::info!("MeiliSearch is healthy.");
        }
        Err(e) => {
            tracing::error!("Failed to ping MeiliSearch.");
            return Err(PingError::MeiliSearchError(e));
        }
    };
    //* ping RDB *//
    match rdb.ping().await {
        Ok(_) => {
            tracing::info!("RDB is healthy.");
        }
        Err(e) => {
            tracing::error!("Failed to ping RDB.");
            return Err(PingError::DbErr(e));
        }
    };
    Ok(())
}

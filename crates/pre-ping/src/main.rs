use domain::repository::connection::ConnectionRepository;
use infrastructure::connection;

#[tokio::main]
async fn main() {
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
            tracing::error!("{}", e.to_string());
            return;
        }
    };
    //* ping RDB *//
    match rdb.ping().await {
        Ok(_) => {
            tracing::info!("RDB is healthy.");
        }
        Err(e) => {
            tracing::error!("Failed to ping RDB.");
            tracing::error!("{}", e.to_string());
            return;
        }
    };
}

use error::init_healthcheck::InitHealthCheckError;
use initialize::initialize;
use migration::migration;

pub mod csv;
pub mod error;
mod initialize;
mod migration;

#[tokio::main]
async fn main() -> Result<(), InitHealthCheckError> {
    // tracing
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(tracing::Level::DEBUG)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;
    // migration
    migration().await?;
    // initialize
    initialize().await?;
    Ok(())
}

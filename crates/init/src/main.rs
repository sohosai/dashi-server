use domain::repository::healthcheck::HealthCheckRepository;
use error::init_healthcheck::InitHealthCheckError;
use infrastructure::healthcheck;
use initialize::initializer;
use migration::migration;

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
    //* Check *//
    let health_check = healthcheck::HealthCheck::new().await;
    // health check and initialize (if not initialized)
    match health_check.healthcheck().await {
        Ok(_) => {
            // already initialized
            tracing::info!("^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^");
            tracing::info!("HealthCheck and Initialize ware passed.");
            tracing::info!("^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^");
        }
        Err(e) => {
            tracing::error!("HealthCheckError: {}", e);
            // not initialized
            // migration
            tracing::info!("^^^^^^^^^^^^^^^^");
            tracing::info!("Migration Start");
            tracing::info!("^^^^^^^^^^^^^^^^");
            migration().await?;
            tracing::info!("^^^^^^^^^^^^^^^^");
            tracing::info!("Migration End");
            tracing::info!("^^^^^^^^^^^^^^^^");
            tracing::info!("^^^^^^^^^^^^^^^^");
            tracing::info!("Initialize Start");
            tracing::info!("^^^^^^^^^^^^^^^^");
            // initialize
            initializer().await?;
            tracing::info!("^^^^^^^^^^^^^^^^");
            tracing::info!("Initialize End");
            tracing::info!("^^^^^^^^^^^^^^^^");
        }
    };
    Ok(())
}

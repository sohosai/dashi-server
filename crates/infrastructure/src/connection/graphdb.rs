use domain::value_object::error::connection::ConnectionError;
use dotenvy::dotenv;
use neo4rs::Graph;
use once_cell::sync::OnceCell;
use std::env;

pub(super) async fn connect_neo4j() -> Result<Graph, ConnectionError> {
    // Set environment variables
    // Declaration and initialization of static variable
    static NEO4J_BOLT_PORT: OnceCell<String> = OnceCell::new();
    static NEO4J_USER: OnceCell<String> = OnceCell::new();
    static NEO4J_PASSWORD: OnceCell<String> = OnceCell::new();
    static NEO4J_HOST: OnceCell<String> = OnceCell::new();
    // load .env file
    dotenv()?;
    // set Object value
    let _ = NEO4J_BOLT_PORT.set(env::var("NEO4J_BOLT_PORT")?);
    let _ = NEO4J_USER.set(env::var("NEO4J_USER")?);
    let _ = NEO4J_PASSWORD.set(env::var("NEO4J_PASSWORD")?);
    let _ = NEO4J_HOST.set(env::var("NEO4J_HOST")?);
    // create Graph instance
    Ok(Graph::new(
        format!(
            "{}:{}",
            NEO4J_HOST
                .get()
                .ok_or(ConnectionError::DotEnvVarNotFountError(
                    "NEO4J_HOST".to_string()
                ))?,
            NEO4J_BOLT_PORT
                .get()
                .ok_or(ConnectionError::DotEnvVarNotFountError(
                    "NEO4J_BOLT_PORT".to_string()
                ))?
        ),
        NEO4J_USER
            .get()
            .ok_or(ConnectionError::DotEnvVarNotFountError(
                "NEO4J_USER".to_string(),
            ))?,
        NEO4J_PASSWORD
            .get()
            .ok_or(ConnectionError::DotEnvVarNotFountError(
                "NEO4J_PASSWORD".to_string(),
            ))?,
    )
    .await?)
}

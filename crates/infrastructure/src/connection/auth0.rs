use domain::value_object::error::connection::ConnectionError;
use dotenvy::dotenv;
use jwtk::jwk::RemoteJwksVerifier;
use once_cell::sync::OnceCell;
use std::{env, time::Duration};

pub async fn connect_auth0() -> Result<RemoteJwksVerifier, ConnectionError> {
    // Set environment variables
    // Declaration and initialization of static variable
    static AUTH0_JSON_WEB_KEY_SET_ENDPOINT: OnceCell<String> = OnceCell::new();
    // load .env file
    dotenv()?;
    // set Object value
    let _ = AUTH0_JSON_WEB_KEY_SET_ENDPOINT.set(env::var("AUTH0_JSON_WEB_KEY_SET_ENDPOINT")?);
    Ok(RemoteJwksVerifier::new(
        AUTH0_JSON_WEB_KEY_SET_ENDPOINT
            .get()
            .ok_or(ConnectionError::DotEnvVarNotFountError(
                "AUTH0_JSON_WEB_KEY_SET_ENDPOINT".to_string(),
            ))?
            .to_string(),
        None,
        Duration::from_secs(3600),
    ))
}

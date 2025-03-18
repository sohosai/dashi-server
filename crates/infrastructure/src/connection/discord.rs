use domain::value_object::error::{
    connection::ConnectionError, discord::collection::DiscordCollection,
};
use dotenvy::dotenv;
use once_cell::sync::OnceCell;
use std::env;

pub async fn connect_discord_item_webhook() -> Result<DiscordCollection, ConnectionError> {
    // Set environment variables
    // Declaration and initialization of static variable
    static DASHI_CLIENT_ENDPOINT: OnceCell<String> = OnceCell::new();
    static DISCORD_ITEM_WEBHOOK_ENDPOINT: OnceCell<String> = OnceCell::new();
    static CLOUDFLARE_R2_IMAGE_URI: OnceCell<String> = OnceCell::new();
    // load .env file
    dotenv()?;
    // set Object value
    let _ = DASHI_CLIENT_ENDPOINT.set(env::var("DASHI_CLIENT_ENDPOINT")?);
    let _ = DISCORD_ITEM_WEBHOOK_ENDPOINT.set(env::var("DISCORD_ITEM_WEBHOOK_ENDPOINT")?);
    let _ = CLOUDFLARE_R2_IMAGE_URI.set(env::var("CLOUDFLARE_R2_IMAGE_URI")?);
    // Create a new Client
    let client = reqwest::Client::new();
    Ok(DiscordCollection {
        dasih_client_endpoint: DASHI_CLIENT_ENDPOINT
            .get()
            .ok_or(ConnectionError::DotEnvVarNotFountError(
                "DASHI_CLIENT_ENDPOINT".to_string(),
            ))?
            .to_string(),
        cloudflare_r2_image_uri: CLOUDFLARE_R2_IMAGE_URI
            .get()
            .ok_or(ConnectionError::DotEnvVarNotFountError(
                "CLOUDFLARE_R2_IMAGE_URI".to_string(),
            ))?
            .to_string(),
        request_builder: client.post(
            DISCORD_ITEM_WEBHOOK_ENDPOINT
                .get()
                .ok_or(ConnectionError::DotEnvVarNotFountError(
                    "DISCORD_ITEM_WEBHOOK_ENDPOINT".to_string(),
                ))?
                .to_string(),
        ),
    })
}

pub async fn connect_discord_rental_webhook() -> Result<DiscordCollection, ConnectionError> {
    // Set environment variables
    // Declaration and initialization of static variable
    static DASHI_CLIENT_ENDPOINT: OnceCell<String> = OnceCell::new();
    static DISCORD_RENTAL_WEBHOOK_ENDPOINT: OnceCell<String> = OnceCell::new();
    static CLOUDFLARE_R2_IMAGE_URI: OnceCell<String> = OnceCell::new();
    // load .env file
    dotenv()?;
    // set Object value
    let _ = DASHI_CLIENT_ENDPOINT.set(env::var("DASHI_CLIENT_ENDPOINT")?);
    let _ = DISCORD_RENTAL_WEBHOOK_ENDPOINT.set(env::var("DISCORD_RENTAL_WEBHOOK_ENDPOINT")?);
    let _ = CLOUDFLARE_R2_IMAGE_URI.set(env::var("CLOUDFLARE_R2_IMAGE_URI")?);
    // Create a new Client
    let client = reqwest::Client::new();
    Ok(DiscordCollection {
        dasih_client_endpoint: DASHI_CLIENT_ENDPOINT
            .get()
            .ok_or(ConnectionError::DotEnvVarNotFountError(
                "DASHI_CLIENT_ENDPOINT".to_string(),
            ))?
            .to_string(),
        cloudflare_r2_image_uri: CLOUDFLARE_R2_IMAGE_URI
            .get()
            .ok_or(ConnectionError::DotEnvVarNotFountError(
                "CLOUDFLARE_R2_IMAGE_URI".to_string(),
            ))?
            .to_string(),
        request_builder: client.post(
            DISCORD_RENTAL_WEBHOOK_ENDPOINT
                .get()
                .ok_or(ConnectionError::DotEnvVarNotFountError(
                    "DISCORD_RENTAL_WEBHOOK_ENDPOINT".to_string(),
                ))?
                .to_string(),
        ),
    })
}

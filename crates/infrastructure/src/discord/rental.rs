use domain::{
    entity::discord::{self, sender::DiscordWebHookSender},
    value_object::error::discord::sender::DiscordWebHookError,
};

pub async fn discord_rental_webhook_sender(
    sender: DiscordWebHookSender,
) -> Result<(), DiscordWebHookError> {
    //* Discord WebHook *//
    let request = discord::json::DiscordWebHookJson {
        embeds: vec![discord::json::Embed {
            title: sender.title,
            description: sender.description,
            color: sender.color,
            url: format!(
                "{}/item/{}",
                sender.connect_discord_rental_webhook.dasih_client_endpoint, sender.item.id
            ),
            image: discord::json::Image {
                url: format!(
                    "{}/{}.webp",
                    sender
                        .connect_discord_rental_webhook
                        .cloudflare_r2_image_uri,
                    sender.item.id
                ),
            },
            fields: vec![
                discord::json::Field {
                    name: "visible_id".to_string(),
                    value: sender.item.visible_id.to_owned(),
                },
                discord::json::Field {
                    name: "name".to_string(),
                    value: sender.item.name.to_owned(),
                },
                discord::json::Field {
                    name: "recipient".to_string(),
                    value: sender.item.recipient.to_owned(),
                },
                discord::json::Field {
                    name: "rental_description".to_string(),
                    value: sender.item.rental_description.to_owned(),
                },
                match sender.item.latest_rent_at {
                    Some(latest_rent_at) => discord::json::Field {
                        name: "latest_rent_at".to_string(),
                        value: latest_rent_at.to_string(),
                    },
                    None => discord::json::Field {
                        name: "latest_rent_at".to_string(),
                        value: "None".to_string(),
                    },
                },
                match sender.item.scheduled_replace_at {
                    Some(scheduled_replace_at) => discord::json::Field {
                        name: "scheduled_replace_at".to_string(),
                        value: scheduled_replace_at.to_string(),
                    },
                    None => discord::json::Field {
                        name: "scheduled_replace_at".to_string(),
                        value: "None".to_string(),
                    },
                },
                match sender.item.latest_replace_at {
                    Some(latest_replace_at) => discord::json::Field {
                        name: "latest_replace_at".to_string(),
                        value: latest_replace_at.to_string(),
                    },
                    None => discord::json::Field {
                        name: "latest_replace_at".to_string(),
                        value: "None".to_string(),
                    },
                },
            ],
            footer: discord::json::Footer {
                text: "dashi-server".to_string(),
            },
        }],
    };
    let response = sender
        .connect_discord_rental_webhook
        .request_builder
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&request)?)
        .send()
        .await?;
    tracing::info!("{:#?}", response);
    Ok(())
}

use domain::{
    entity::discord::{self, sender::DiscordWebHookSender},
    value_object::error::discord::sender::DiscordWebHookError,
};

pub async fn discord_rental_webhook_sender(
    sender: DiscordWebHookSender,
) -> Result<(), DiscordWebHookError> {
    //* Discord WebHook *//
    let mut fields = vec![
        discord::json::Field {
            name: "ラベルID".to_string(),
            value: sender.item.visible_id.to_owned(),
        },
        discord::json::Field {
            name: "物品名".to_string(),
            value: sender.item.name.to_owned(),
        },
    ];
    if let Some(latest_rent_at) = sender.item.latest_rent_at {
        fields.push(discord::json::Field {
            name: "借用者".to_string(),
            value: sender.item.recipient.to_owned(),
        });
        fields.push(discord::json::Field {
            name: "貸し出し中に関する備考".to_string(),
            value: sender.item.rental_description.to_owned(),
        });
        fields.push(discord::json::Field {
            name: "最終貸し出し日時 (UTC)".to_string(),
            value: latest_rent_at.to_string(),
        });
    };
    if let Some(scheduled_replace_at) = sender.item.scheduled_replace_at {
        fields.push(discord::json::Field {
            name: "返却予定日時 (JST)".to_string(),
            value: scheduled_replace_at.to_string(),
        })
    };
    if let Some(latest_replace_at) = sender.item.latest_replace_at {
        fields.push(discord::json::Field {
            name: "最終返却日時 (UTC)".to_string(),
            value: latest_replace_at.to_string(),
        })
    };
    let request = discord::json::DiscordWebHookJson {
        embeds: vec![discord::json::Embed {
            title: sender.title,
            description: sender.description,
            color: sender.color,
            url: format!(
                "{}/item/{}",
                sender.connect_discord_webhook.dasih_client_endpoint, sender.item.id
            ),
            image: discord::json::Image {
                url: format!(
                    "{}/{}.webp",
                    sender.connect_discord_webhook.cloudflare_r2_image_uri, sender.item.id
                ),
            },
            fields,
            footer: discord::json::Footer {
                text: "dashi-server".to_string(),
            },
        }],
    };
    let response = sender
        .connect_discord_webhook
        .request_builder
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&request)?)
        .send()
        .await?;
    tracing::info!("{:#?}", response);
    Ok(())
}

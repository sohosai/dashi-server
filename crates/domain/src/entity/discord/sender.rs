use crate::value_object::error::discord::collection::DiscordCollection;

pub struct DiscordWebHookSender {
    pub title: String,
    pub description: String,
    pub color: i32,
    pub item: entity::item::Model,
    pub connect_discord_rental_webhook: DiscordCollection,
}

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct DiscordWebHookJson {
    pub embeds: Vec<Embed>,
}

#[derive(Debug, Serialize)]
pub struct Embed {
    pub title: String,
    pub description: String,
    pub url: String,
    pub color: i32,
    pub image: Image,
    pub fields: Vec<Field>,
    pub footer: Footer,
}

#[derive(Debug, Serialize)]
pub struct Field {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Serialize)]
pub struct Image {
    pub url: String,
}

#[derive(Debug, Serialize)]
pub struct Footer {
    pub text: String,
}

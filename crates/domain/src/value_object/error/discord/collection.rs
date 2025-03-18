use reqwest::RequestBuilder;

pub struct DiscordCollection {
    pub dasih_client_endpoint: String,
    pub cloudflare_r2_image_uri: String,
    pub request_builder: RequestBuilder,
}

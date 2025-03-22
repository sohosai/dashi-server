use presentation::api::api;

#[tokio::main]
async fn main() {
    let _ = api().await;
}

use presentation::api::ApiDoc;
use utoipa::OpenApi;

fn main() {
    let docs = generate_openapi();
    print!("{docs}");
}

fn generate_openapi() -> String {
    ApiDoc::openapi().to_yaml().unwrap()
}

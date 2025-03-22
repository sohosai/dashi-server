use presentation::api::ApiDoc;
use std::fs::File;
use std::io::Write;
use utoipa::OpenApi;

fn main() {
    let mut file = File::create("schema.yaml").expect("schema.yaml was not found.");
    let docs = generate_openapi();
    writeln!(file, "{}", docs).expect("Failed to write schema.yaml");
    file.flush().expect("Failed to flush schema.yaml");
}

fn generate_openapi() -> String {
    ApiDoc::openapi().to_yaml().unwrap()
}

use std::env;

use dotenvy::dotenv;
use once_cell::sync::OnceCell;
use tokio::process::Command;

use crate::error::migration::MigrationError;

pub(super) async fn migration() -> Result<(), MigrationError> {
    match Command::new("bash")
        .arg("-c")
        .arg("/app/target/release/migration")
        .output()
        .await
    {
        Ok(output) => {
            println!("{}", String::from_utf8_lossy(&output.stdout));
        }
        Err(e) => {
            return Err(MigrationError::RunShellScriptError(e.to_string()));
        }
    };
    Ok(())
}
// pub(super) async fn migration() -> Result<(), MigrationError> {
//     // Migration
//     // source /app/.env
//     match Command::new("bash")
//         .arg("-c")
//         .arg("source /app/.env")
//         .output()
//         .await
//     {
//         Ok(output) => {
//             println!("{}", String::from_utf8_lossy(&output.stdout));
//         }
//         Err(e) => {
//             return Err(MigrationError::RunShellScriptError(e.to_string()));
//         }
//     };

//     // cargo run --manifest-path /app/migration/Cargo.toml -- refresh -u postgres://{POSTGRES_USER}:{POSTGRES_PASSWORD}@{POSTGRES_HOST}:{POSTGRES_PORT}/{POSTGRES_DB}
//     // Declaration and initialization of static variable
//     static POSTGRES_USER: OnceCell<String> = OnceCell::new();
//     static POSTGRES_PASSWORD: OnceCell<String> = OnceCell::new();
//     static POSTGRES_PORT: OnceCell<String> = OnceCell::new();
//     static POSTGRES_DB: OnceCell<String> = OnceCell::new();
//     static POSTGRES_HOST: OnceCell<String> = OnceCell::new();
//     // load .env file
//     match dotenv() {
//         Ok(_) => {}
//         Err(e) => {
//             tracing::error!("Failed to load .env file.");
//             return Err(MigrationError::DotenvyError(e));
//         }
//     }
//     // set Object value
//     match env::var("POSTGRES_USER") {
//         Ok(postgres_user) => {
//             let _ = POSTGRES_USER.set(postgres_user);
//         }
//         Err(e) => {
//             tracing::error!("Failed to get POSTGRES_USER.");
//             return Err(MigrationError::VarError(e));
//         }
//     };
//     match env::var("POSTGRES_PASSWORD") {
//         Ok(postgres_password) => {
//             let _ = POSTGRES_PASSWORD.set(postgres_password);
//         }
//         Err(e) => {
//             tracing::error!("Failed to get POSTGRES_PASSWORD.");
//             return Err(MigrationError::VarError(e));
//         }
//     }
//     match env::var("POSTGRES_PORT") {
//         Ok(postgres_port) => {
//             let _ = POSTGRES_PORT.set(postgres_port);
//         }
//         Err(e) => {
//             tracing::error!("Failed to get POSTGRES_PORT.");
//             return Err(MigrationError::VarError(e));
//         }
//     }
//     match env::var("POSTGRES_DB") {
//         Ok(postgres_db) => {
//             let _ = POSTGRES_DB.set(postgres_db);
//         }
//         Err(e) => {
//             tracing::error!("Failed to get POSTGRES_DB.");
//             return Err(MigrationError::VarError(e));
//         }
//     }
//     match env::var("POSTGRES_HOST") {
//         Ok(postgres_host) => {
//             let _ = POSTGRES_HOST.set(postgres_host);
//         }
//         Err(e) => {
//             tracing::error!("Failed to get POSTGRES_HOST.");
//             return Err(MigrationError::VarError(e));
//         }
//     }
//     let command = format!(
//         "cargo run --manifest-path /app/migration/Cargo.toml -- refresh -u postgres://{}:{}@{}:{}/{}",
//         POSTGRES_USER.get().ok_or(MigrationError::VarGetError("POSTGRES_USER".to_string()))?,
//         POSTGRES_PASSWORD.get().ok_or(MigrationError::VarGetError("POSTGRES_PASSWORD".to_string()))?,
//         POSTGRES_HOST.get().ok_or(MigrationError::VarGetError("POSTGRES_HOST".to_string()))?,
//         POSTGRES_PORT.get().ok_or(MigrationError::VarGetError("POSTGRES_PORT".to_string()))?,
//         POSTGRES_DB.get().ok_or(MigrationError::VarGetError("POSTGRES_DB".to_string()))?
//     );
//     match Command::new("bash").arg("-c").arg(command).output().await {
//         Ok(output) => {
//             println!("{}", String::from_utf8_lossy(&output.stdout));
//         }
//         Err(e) => {
//             return Err(MigrationError::RunShellScriptError(e.to_string()));
//         }
//     };
//     Ok(())
// }

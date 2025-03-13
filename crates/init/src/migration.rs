use crate::error::migration::MigrationError;
use tokio::process::Command;

pub(super) async fn migration() -> Result<(), MigrationError> {
    match Command::new("bash")
        .arg("-c")
        .arg("/app/target/release/migration")
        .output()
        .await
    {
        Ok(output) => {
            tracing::info!("Migration Result");
            println!("{}", String::from_utf8_lossy(&output.stdout));
        }
        Err(e) => {
            return Err(MigrationError::RunShellScriptError(e.to_string()));
        }
    };
    Ok(())
}

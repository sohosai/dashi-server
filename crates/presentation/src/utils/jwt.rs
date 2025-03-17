use crate::models::rwlock_shared_state::RwLockSharedState;
use axum::{
    extract::{Request, State},
    middleware::Next,
    response::IntoResponse,
};
use domain::value_object::error::AppError;
use infrastructure::connection::auth0::connect_auth0;
use serde_json::{Map, Value};

pub async fn jwt_middleware(
    State(_state): State<RwLockSharedState>,
    request: Request,
    next: Next,
) -> Result<impl IntoResponse, AppError> {
    let remote_jwks_verifier = connect_auth0().await?;
    //* preprocess *//
    tracing::info!("Preprocess");
    tracing::info!(
        "Method: {}, URI: {}, headers: {:?}, request: {:?}",
        request.method(),
        request.uri(),
        request.headers(),
        request.body()
    );
    // get JWT
    let token = request
        .headers()
        .get("Authorization")
        .and_then(|auth_header| auth_header.to_str().ok())
        .and_then(|auth_value| {
            if auth_value.starts_with("Bearer ") {
                auth_value.strip_prefix("Bearer ").map(|s| s.to_owned())
            } else {
                None
            }
        });

    match token {
        Some(token) => {
            // check JWT
            match remote_jwks_verifier
                .verify::<Map<String, Value>>(&token)
                .await
            {
                Ok(_) => {
                    tracing::info!("Token is valid");
                }
                Err(err) => {
                    tracing::info!("Token is invalid");
                    tracing::info!("{}", err);
                }
            }
        }
        None => {
            tracing::info!("Token is not found");
        }
    }
    //* handler *//
    tracing::info!("Handler");
    let response = next.run(request).await;
    //* postprocess *//
    tracing::info!("Postprocess");
    tracing::info!(
        "Status: {}, headers: {:?}, request: {:?}",
        response.status(),
        response.headers(),
        response.body()
    );
    Ok(response)

    // match token {
    //     Some(token) => {
    //         // check JWT
    //         match remote_jwks_verifier
    //             .verify::<Map<String, Value>>(&token)
    //             .await
    //         {
    //             Ok(_) => {
    //                 tracing::info!("Token is valid");
    //                 //* handler *//
    //                 tracing::info!("Handler");
    //                 let response = next.run(request).await;
    //                 //* postprocess *//
    //                 tracing::info!("Postprocess");
    //                 tracing::info!(
    //                     "Status: {}, headers: {:?}, request: {:?}",
    //                     response.status(),
    //                     response.headers(),
    //                     response.body()
    //                 );
    //                 Ok(response)
    //             }
    //             Err(err) => {
    //                 tracing::info!("Token is invalid");
    //                 tracing::info!("{}", err);
    //                 Ok((StatusCode::UNAUTHORIZED, ()).into_response())
    //             }
    //         }
    //     }
    //     None => Ok((StatusCode::FORBIDDEN, ()).into_response()),
    // }
}

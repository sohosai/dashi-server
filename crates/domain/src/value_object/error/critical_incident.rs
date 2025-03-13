//This func is used to rollback the transaction in case of any error
pub async fn rollback_error() {
    tracing::error!("^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^");
    tracing::error!("Rollback Error: A critical incident has occurred.");
    tracing::error!("^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^");
}

pub async fn conflict_error() {
    tracing::error!("^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^");
    tracing::error!("Conflict Error: A critical incident has occurred.");
    tracing::error!("^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^");
}

pub async fn multiple_parent_items_error() {
    tracing::error!("^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^");
    tracing::error!("Multiple Parent Items Error: A critical incident has occurred.");
    tracing::error!("^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^");
}

pub async fn parent_item_missing_error() {
    tracing::error!("^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^");
    tracing::error!("Parent Item Missing Error: A critical incident has occurred.");
    tracing::error!("^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^");
}

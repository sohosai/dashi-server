use async_std::sync::RwLock;
use infrastructure::shared_state::SharedState;
use std::sync::Arc;

//RwLockSharedState
pub type RwLockSharedState = Arc<RwLock<SharedState>>;

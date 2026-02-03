#[cfg(feature = "server")]
pub type State = axum::extract::Extension<std::sync::Arc<infra::setup::AppState>>;

#[cfg(not(feature = "server"))]
pub struct State;

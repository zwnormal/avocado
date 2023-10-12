use crate::cmd::Command;
use crate::err::AppError;
use crate::session::Session;
use crate::state::State as AppState;
use crate::user::cmd::logout::Logout;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Extension;

#[tracing::instrument(name = "Calling 'user logout' api", skip(session, state))]
pub(crate) async fn logout(
    Extension(session): Extension<Session>,
    State(state): State<AppState>,
) -> Result<Response, AppError> {
    Logout { session }.execute(state.clone()).await?;
    Ok(StatusCode::OK.into_response())
}

use crate::cmd::Command;
use crate::err::AppError;
use crate::state::State as AppState;
use crate::user::cmd::login::Login;
use axum::extract::State;
use axum::response::{IntoResponse, Response};
use axum::Json;
use axum_extra::extract::WithRejection;
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize, Debug)]
pub(crate) struct LoginReply {
    session_id: Uuid,
}

#[tracing::instrument(name = "Calling 'user login' api", skip(state))]
pub(crate) async fn login(
    State(state): State<AppState>,
    WithRejection(Json(login), _): WithRejection<Json<Login>, AppError>,
) -> Result<Response, AppError> {
    let session_id = login.execute(state.clone()).await?;
    Ok(Json(LoginReply { session_id }).into_response())
}

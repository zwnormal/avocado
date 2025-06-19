use crate::cmd::Command;
use crate::err::JsonError;
use crate::session::Session;
use crate::state::State as AppState;
use crate::user::cmd::list::List;
use avocado_proto::grpc::user::UserReply;
use axum::extract::State;
use axum::{Extension, Json};

#[tracing::instrument(name = "Calling 'user list' api", skip(session, state))]
pub(crate) async fn list(
    Extension(session): Extension<Session>,
    State(state): State<AppState>,
) -> Result<Json<Vec<UserReply>>, JsonError> {
    let list_reply = List { session }.execute(state.clone()).await?;
    Ok(Json(list_reply))
}

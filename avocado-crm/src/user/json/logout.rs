use crate::cmd::Command;
use crate::err::AppError;
use crate::session::Session;
use crate::state::State as AppState;
use crate::user::cmd::logout::Logout;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Extension;
use axum_extra::extract::cookie::Cookie;
use axum_extra::extract::CookieJar;

#[tracing::instrument(name = "Calling 'user logout' api", skip(session, state))]
pub(crate) async fn logout(
    cookie: CookieJar,
    Extension(session): Extension<Session>,
    State(state): State<AppState>,
) -> Result<Response, AppError> {
    let cookie = cookie.remove(Cookie::named("session_id"));
    Logout { session }.execute(state.clone()).await?;
    Ok((cookie, StatusCode::OK.into_response()).into_response())
}

use crate::err::AppError;
use crate::session::cmd::refresh_token::RefreshToken;
use crate::session::SessionId;
use crate::state::State as AppState;
use axum::extract::State;
use axum::http::{Request, StatusCode};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use axum::Json;
use axum_extra::extract::cookie::{Cookie, CookieJar};
use serde_json::json;
use std::str::FromStr;
use uuid::Uuid;

pub(crate) async fn auth<B>(
    State(state): State<AppState>,
    cookie: CookieJar,
    mut req: Request<B>,
    next: Next<B>,
) -> Result<Response, AppError> {
    let path = req.uri().path();
    if path != "/api/user/login" {
        tracing::info!("receiving cookie {:?}", cookie);
        match get_session_id(&cookie) {
            Some(session_id) => match state.session_store.get(&session_id).await {
                Ok(Some(session))
                    if RefreshToken {
                        session: session.clone(),
                    }
                    .execute(state.clone())
                    .await
                    .is_ok() =>
                {
                    req.extensions_mut().insert(session);
                    Ok(next.run(req).await)
                }
                _ => clear_session(state.clone(), Some(&session_id), cookie).await,
            },
            None => clear_session(state.clone(), None, cookie).await,
        }
    } else {
        Ok(next.run(req).await)
    }
}

fn get_session_id(cookie: &CookieJar) -> Option<Uuid> {
    cookie
        .get("session_id")
        .and_then(|c| Uuid::from_str(c.value()).ok())
}

async fn clear_session(
    state: AppState,
    session_id: Option<&SessionId>,
    cookie: CookieJar,
) -> Result<Response, AppError> {
    // Delete the session id from database
    if let Some(session_id) = session_id {
        match state.session_store.delete(session_id).await {
            Ok(_) => {}
            Err(e) => tracing::warn!("failed to delete session id: {:?}", e),
        }
    }
    // Clear the session id from cookie
    let cookie = cookie.remove(Cookie::named("session_id"));
    // Return the unauthorized message
    Ok((
        StatusCode::UNAUTHORIZED,
        cookie,
        Json(json!({"error": "user is unauthenticated"})),
    )
        .into_response())
}

use crate::err::AppError;
use crate::session::cmd::refresh_token::RefreshToken;
use crate::session::SessionId;
use crate::state::State as AppState;
use axum::extract::State;
use axum::http::header::AUTHORIZATION;
use axum::http::{HeaderMap, HeaderValue, Request, StatusCode};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;
use std::str::FromStr;
use uuid::Uuid;

pub(crate) async fn auth<B>(
    State(state): State<AppState>,
    mut req: Request<B>,
    next: Next<B>,
) -> Result<Response, AppError> {
    let path = req.uri().path();
    if path != "/api/user/login" {
        match get_session_id(req.headers()) {
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
                _ => clear_session(state.clone(), Some(&session_id)).await,
            },
            None => clear_session(state.clone(), None).await,
        }
    } else {
        Ok(next.run(req).await)
    }
}

fn get_session_id(headers: &HeaderMap<HeaderValue>) -> Option<Uuid> {
    let authorization_header = match headers.get(AUTHORIZATION) {
        Some(v) => v,
        None => return None,
    };

    let authorization = match authorization_header.to_str() {
        Ok(v) => v,
        Err(_) => return None,
    };

    if !authorization.starts_with("Bearer ") {
        return None;
    }

    let session_id = authorization.trim_start_matches("Bearer ");
    Uuid::from_str(session_id).ok()
}

async fn clear_session(
    state: AppState,
    session_id: Option<&SessionId>,
) -> Result<Response, AppError> {
    // Delete the session id from database
    if let Some(session_id) = session_id {
        match state.session_store.delete(session_id).await {
            Ok(_) => {}
            Err(e) => tracing::warn!("failed to delete session id: {:?}", e),
        }
    }
    // Return the unauthorized message
    Ok((
        StatusCode::UNAUTHORIZED,
        Json(json!({"error": "user is unauthenticated"})),
    )
        .into_response())
}
